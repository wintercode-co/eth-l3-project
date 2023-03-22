use std::sync::Arc;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use futures::stream::StreamExt;

use crate::config::{connect_http, connect_ws};

mod config;

abigen!(Rollup, "out/Rollup.sol/Rollup.json");
abigen!(RollupBridge, "out/Rollup.sol/RollupBridge.json");

#[tokio::main]
async fn main() {
    let conf = config::Config::new();

    let scroll_zkevm_prov = connect_http(&conf.scroll_zkevm.rpc_url);
    let rollup_config_prov = connect_ws(&conf.rollup_config.rollup_network.rpc_url).await;

    let signer_wallet = conf.get_signer();
    let signer = Arc::new(SignerMiddleware::new(
        scroll_zkevm_prov,
        signer_wallet.with_chain_id(conf.scroll_zkevm.chain_id),
    ));

    let rollup_bridge_contract = RollupBridge::new(
        conf.rollup_config.rollup_bridge_address.clone(),
        Arc::clone(&signer),
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
        post_transactions_to_rollup(&transactions, &rollup_bridge_contract).await;
    }
}

async fn post_transactions_to_rollup(
    transactions: &Vec<H256>,
    rollup_bridge_contract: &RollupBridge<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) {
    let tx = rollup_bridge_contract.submit_transactions(Bytes::new());
    tx.send()
        .await
        .expect("Failed to submit rollup transactions");
}
