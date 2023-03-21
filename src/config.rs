use std::fs;

use ethers::prelude::LocalWallet;
use ethers::providers::{Http, JsonRpcClient, Provider, Ws};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkConfig {
    pub rpc_url: String,
}

pub fn connect_http(rpc_url: &str) -> Provider<Http> {
    Provider::<Http>::try_from(rpc_url).expect("Unable to connect to provider")
}

pub async fn connect_ws(rpc_url: &str) -> Provider<Ws> {
    Provider::<Ws>::new(Ws::connect(rpc_url).await.expect("Unable to connect to provider"))
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RollupConfig {
    pub rollup_network: NetworkConfig,
    batch_size: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub polygon_zkevm: NetworkConfig,
    pub scroll_zkevm: NetworkConfig,
    pub rollup_config: RollupConfig,
    account_pk: String,
}

impl Config {
    pub fn new() -> Self {
        let network_file = fs::read_to_string("config.testnet.yaml").expect("Unable to read file");
        serde_yaml::from_str(&network_file).expect("Unable to parse config file")
    }

    pub fn get_signer(&self) -> LocalWallet {
        self.account_pk
            .parse::<LocalWallet>()
            .expect("Unable to generate wallet")
    }
}
