use std::fmt::Error;
use std::{convert::TryFrom, path::Path, sync::Arc};

use ethers::{
    contract::{abigen, ContractFactory},
    core::utils::Anvil,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    solc::Solc,
    types::{Address, U256},
    utils::AnvilInstance,
};

use crate::config::NetworkConfig;

pub(crate) async fn deploy_contract(
    network_config: &NetworkConfig,
    deployer_wallet: LocalWallet,
    contract_name: &str,
) -> Result<Address, Error> {
    let source =
        Path::new(&env!("CARGO_MANIFEST_DIR")).join(format!("contracts/{}.sol", contract_name));
    let compiled = Solc::default()
        .compile_source(source)
        .expect("Could not compile contract");
    let (abi, bytecode, _runtime_bytecode) = compiled
        .find(contract_name)
        .expect("could not find contract")
        .into_parts_or_default();

    let provider = Provider::<Http>::try_from(&network_config.rpc_url).unwrap();

    let client = SignerMiddleware::new(
        provider,
        deployer_wallet.with_chain_id(network_config.chain_id),
    );
    let client = Arc::new(client);
    let factory = ContractFactory::new(abi, bytecode, client.clone());
    let contract = factory.deploy(()).unwrap().send().await.unwrap();

    Ok(contract.address())
}
