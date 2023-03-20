use std::fs;

use ethers::prelude::{LocalWallet, Wallet};
use ethers::types::PathOrString::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkConfig {
    pub rpc_urc: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub polygon_zkevm: NetworkConfig,
    pub scroll_zkevm: NetworkConfig,
    account_pk: String,
}

impl Config {
    pub fn new() -> Self {
        let network_file = fs::read_to_string("config.testnet.yaml").expect("Unable to read file");
        serde_yaml::from_str(&network_file).expect("Unable to parse config file")
    }

    pub fn get_signer(&self) -> LocalWallet {
        self.account_pk.parse::<LocalWallet>().expect("Unable to generate wallet")
    }
}
