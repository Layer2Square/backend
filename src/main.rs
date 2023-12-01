use ethers::prelude::*;

mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = client::get_goerli_client().await;
    let block_number: U64 = provider.get_block_number().await?;
    println!("Current Block Number is {block_number}");

    Ok(())
}