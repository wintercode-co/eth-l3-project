use ethers::prelude::*;
use futures::stream::StreamExt;

use crate::config::{connect_http, connect_ws};

mod config;

#[tokio::main]
async fn main() {
    let conf = config::Config::new();

    let scroll_zkevm_prov = connect_http(&conf.scroll_zkevm.rpc_url);
    let rollup_config_prov = connect_ws(&conf.rollup_config.rollup_network.rpc_url).await;


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
        post_transactions_to_rollup(&transactions);
    }

    /*
    let signer = conf.get_signer();
    let signer_address = signer.address();
    let signer_balance = scroll_zkevm_prov.get_balance(signer_address, Option::None).await.unwrap();
    println!("Signer Address {:?}", signer_address);
    println!("Signer balance {:?}", signer_balance);

    */
}

fn post_transactions_to_rollup(transactions: &Vec<H256>) {
    unimplemented!()
}
