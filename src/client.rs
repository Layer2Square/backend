use std::env;

use dotenv::dotenv;
use ethers::prelude::*;

pub async fn get_goerli_client() -> Provider<Http> {
    dotenv().ok();
    let goerli_url: String = env::var("GOERLI_URL").expect("GOERLI_URL must be set");
    let rpc_url: &str = &goerli_url.as_str();
    
    Provider::<Http>::try_from(rpc_url).unwrap()
}

pub async fn get_op_goerli_client() -> Provider<Http> {
    dotenv().ok();
    let goerli_url: String = env::var("OPGOERLI_URL").expect("OPGOERLI_URL must be set");
    let rpc_url: &str = &goerli_url.as_str();

    Provider::<Http>::try_from(rpc_url).unwrap()
}

pub async fn get_arb_goerli_client() -> Provider<Http> {
    dotenv().ok();
    let goerli_url: String = env::var("ARBGORLI_URL").expect("ARBGORLI_URL must be set");
    let rpc_url: &str = &goerli_url.as_str();

    Provider::<Http>::try_from(rpc_url).unwrap()
}

pub async fn get_base_goerli_client() -> Provider<Http> {
    dotenv().ok();
    let goerli_url: String = env::var("BASEGOERLI_URL").expect("BASEGOERLI_URL must be set");
    let rpc_url: &str = &goerli_url.as_str();

    Provider::<Http>::try_from(rpc_url).unwrap()
}

pub async fn get_zksync_goerli_client() -> Provider<Http> {
    dotenv().ok();
    let goerli_url: String = env::var("ZKSYNC_URL").expect("ZKSYNC_URL must be set");
    let rpc_url: &str = &goerli_url.as_str();

    Provider::<Http>::try_from(rpc_url).unwrap()
}

pub async fn get_zkevm_goerli_client() -> Provider<Http> {
    dotenv().ok();
    let goerli_url: String = env::var("ZKEVM_URL").expect("ZKEVM_URL must be set");
    let rpc_url: &str = &goerli_url.as_str();

    Provider::<Http>::try_from(rpc_url).unwrap()
}

pub async fn get_scroll_sepolia_client() -> Provider<Http> {
    dotenv().ok();
    let goerli_url: String = env::var("SCROLL_URL").expect("SCROLL_URL must be set");
    let rpc_url: &str = &goerli_url.as_str();

    Provider::<Http>::try_from(rpc_url).unwrap()
}