package main

import (
	"context"
	"crypto/ecdsa"
	"fmt"
	"log"
	"math/big"
	"os"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"

	store "github.com/wintercode-co/eth-l3-project/contracts"
)

const POLYGON_ZK_EVM_TEST_NET_RPC = "https://rpc.public.zkevm-test.net"

var POLYGON_ZK_EVM_TEST_NET_ACCOUNT string

// Dev only. Use a test account for this since this isn't very secure. Remove it before publishing
var POLYGON_ZK_EVM_TEST_ACCOUNT_KEY string

func main() {
	POLYGON_ZK_EVM_TEST_NET_ACCOUNT = os.Getenv("POLYGON_ZK_EVM_TEST_NET_ACCOUNT")
	POLYGON_ZK_EVM_TEST_ACCOUNT_KEY = os.Getenv("POLYGON_ZK_EVM_TEST_ACCOUNT_KEY")
	client, err := ethclient.Dial(POLYGON_ZK_EVM_TEST_NET_RPC)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("we have a connection")
	_ = client

	account := common.HexToAddress(POLYGON_ZK_EVM_TEST_NET_ACCOUNT)
	balance, err := client.BalanceAt(context.Background(), account, nil)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(balance)

	pendingBalance, err := client.PendingBalanceAt(context.Background(), account)
	fmt.Println(pendingBalance)

	// This is a test account - no worries if this gets leaked
	privateKey, err := crypto.HexToECDSA(POLYGON_ZK_EVM_TEST_ACCOUNT_KEY)
	if err != nil {
		log.Fatal(err)
	}

	publicKey := privateKey.Public()
	publicKeyECDSA, ok := publicKey.(*ecdsa.PublicKey)
	if !ok {
		log.Fatal("error casting public key to ECDSA")
	}

	fromAddress := crypto.PubkeyToAddress(*publicKeyECDSA)
	nonce, err := client.PendingNonceAt(context.Background(), fromAddress)
	if err != nil {
		log.Fatal(err)
	}

	gasPrice, err := client.SuggestGasPrice(context.Background())
	if err != nil {
		log.Fatal(err)
	}

	auth := bind.NewKeyedTransactor(privateKey)
	auth.Nonce = big.NewInt(int64(nonce))
	auth.Value = big.NewInt(0)     // in wei
	auth.GasLimit = uint64(300000) // in units
	auth.GasPrice = gasPrice

	input := "1.0"
	address, tx, instance, err := store.DeployStore(auth, client, input)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(address.Hex())
	fmt.Println(tx.Hash().Hex())

	_ = instance
}
