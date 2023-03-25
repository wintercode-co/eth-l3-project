use std::sync::Arc;
use std::time::Duration;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::providers::HttpClientError::JsonRpcError;
use futures::stream::StreamExt;
use tokio::time::{sleep, Sleep};

use crate::config::{connect_http, connect_ws};
use crate::deploy::deploy_contract;

mod config;
mod deploy;

abigen!(Rollup, "out/Rollup.sol/Rollup.json");
abigen!(SimpleRollup, "out/SimpleRollup.sol/Rollup.json");
abigen!(RollupBridge, "out/Rollup.sol/RollupBridge.json");

enum TransactionResult {
    Success,
    Fail(bool),
}

#[tokio::main]
async fn main() {
    let conf = config::Config::new();

    let scroll_zkevm_prov = connect_http(&conf.scroll_zkevm.rpc_url);
    let rollup_config_prov = connect_ws(&conf.rollup_config.rollup_network.rpc_url).await;

    let signer_wallet = conf.scroll_zkevm.get_signer();
    let zk_scroll_signer = Arc::new(SignerMiddleware::new(
        scroll_zkevm_prov,
        signer_wallet.with_chain_id(conf.scroll_zkevm.chain_id),
    ));

    let rollup_bridge_contract = RollupBridge::new(
        conf.scroll_zkevm.rollup_bridge_address.unwrap().clone(),
        Arc::clone(&zk_scroll_signer),
    );

    let mut stream = rollup_config_prov
        .subscribe_blocks()
        .await
        .unwrap()
        .chunks(conf.rollup_config.batch_size.into());
    let mut transactions: Vec<H256> = vec![];

    while let Some(blocks) = stream.next().await {
        for mut block in blocks {
            transactions.append(&mut block.transactions);
        }
        loop {
            match post_transactions_to_rollup(&transactions, &rollup_bridge_contract).await {
                TransactionResult::Success => continue,
                TransactionResult::Fail(retry) => {
                    if retry == true {
                        sleep(Duration::from_secs(120)).await;
                        post_transactions_to_rollup(&transactions, &rollup_bridge_contract).await;
                    }
                }
            }
        }
    }
}

async fn post_transactions_to_rollup(
    transactions: &Vec<H256>,
    rollup_bridge_contract: &RollupBridge<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) -> TransactionResult {
    let tx = rollup_bridge_contract.submit_transactions(Bytes::new());
    let response = tx.send().await;

    return match response {
        Ok(..) => TransactionResult::Success,
        Err(err) => match err {
            ContractError::ProviderError { e } => match e {
                ProviderError::JsonRpcClientError(rpc_error) => TransactionResult::Fail(true),
                _ => TransactionResult::Fail(false),
            },
            _ => TransactionResult::Fail(false),
        },
    };
}
