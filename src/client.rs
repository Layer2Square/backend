use std::env;

use dotenv::dotenv;
use ethers::prelude::*;

pub async fn get_goerli_client() -> Provider<Http> {
    dotenv().ok();
    let goerli_url: String = env::var("GOERLI_URL").expect("GOERLI_URL must be set");
    // let provider = Provider::<Http>::try_from(goerli_url)?;
    // Ok(provider)
    let rpc_url: &str = &goerli_url.as_str();

    Provider::<Http>::connect(rpc_url).await.unwrap()
}

pub async fn get_opGoerli_client() -> Provider<Http> {
    dotenv().ok();
    let goerli_url: String = env::var("OPGOERLI_URL").expect("OPGOERLI_URL must be set");
    let rpc_url: &str = &goerli_url.as_str();

    Provider::<Http>::connect(rpc_url).await.unwrap()
}