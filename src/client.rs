use std::env;

use dotenv::dotenv;
use ethers::prelude::*;

pub async fn get_sepolia_client() -> Provider<Http> {
    dotenv().ok();
    let sepolia_url: String = env::var("SEPOLIA_URL").expect("SEPOLIA_URL must be set");
    let rpc_url: &str = &&sepolia_url.as_str();
    
    Provider::<Http>::try_from(rpc_url).unwrap()
}

pub async fn get_op_sepolia_client() -> Provider<Http> {
    dotenv().ok();
    let op_sepolia_url: String = env::var("OPSEPOLIA_URL").expect("OPSEPOLIA_URL must be set");
    let rpc_url: &str = &&op_sepolia_url.as_str();
    
    Provider::<Http>::try_from(rpc_url).unwrap()
}

pub async fn get_arb_sepolia_client() -> Provider<Http> {
    dotenv().ok();
    let arb_sepolia_url: String = env::var("ARBSEPOLIA_URL").expect("ARBSEPOLIA_URL must be set");
    let rpc_url: &str = &&arb_sepolia_url.as_str();
    
    Provider::<Http>::try_from(rpc_url).unwrap()
}

pub async fn get_base_sepolia_client() -> Provider<Http> {
    dotenv().ok();
    let goerli_url: String = env::var("BASESEPOLIA_URL").expect("BASESEPOLIA_URL must be set");
    let rpc_url: &str = &goerli_url.as_str();

    Provider::<Http>::try_from(rpc_url).unwrap()
}

pub async fn get_zksync_goerli_client() -> Provider<Http> {
    dotenv().ok();
    let goerli_url: String = env::var("ZKSYNC_URL").expect("ZKSYNC_URL must be set");
    let rpc_url: &str = &goerli_url.as_str();

    Provider::<Http>::try_from(rpc_url).unwrap()
}

pub async fn get_zkevm_sepolia_client() -> Provider<Http> {
    dotenv().ok();
    let goerli_url: String = env::var("ZKEVM_SEPOLIA_URL").expect("ZKEVM_SEPOLIA_URL must be set");
    let rpc_url: &str = &goerli_url.as_str();

    Provider::<Http>::try_from(rpc_url).unwrap()
}

pub async fn get_scroll_sepolia_client() -> Provider<Http> {
    dotenv().ok();
    let goerli_url: String = env::var("SCROLL_URL").expect("SCROLL_URL must be set");
    let rpc_url: &str = &goerli_url.as_str();

    Provider::<Http>::try_from(rpc_url).unwrap()
}