use ethers::prelude::*;

mod config;

#[tokio::main]
async fn main() {
    let conf = config::Config::new();

    let polygon_zkevm_prov = Provider::<Http>::try_from(&conf.polygon_zkevm.rpc_urc)
        .expect("Unable to connect to provider");
    let scroll_zkevm_prov = Provider::<Http>::try_from(&conf.scroll_zkevm.rpc_urc)
        .expect("Unable to connect to provider");


    let signer = conf.get_signer();

    let signer_address = signer.address();
    let signer_balance = scroll_zkevm_prov.get_balance(signer_address, Option::None).await.unwrap();
    println!("Signer Address {:?}", signer_address);
    println!("Signer balance {:?}", signer_balance);
}
