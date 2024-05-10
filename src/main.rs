use std::sync::Arc;
type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;
use ethers::{abi::Address, prelude::*, providers::Provider};

mod client;
mod call_function;
mod wallet;

mod abi;
use abi::{ONFT1155NFT, ONFT721NFT};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = client::get_sepolia_client().await;
    let block_number: U64 = provider.get_block_number().await?;
    println!("Current Block Number is {block_number}");

    let arb_provider = client::get_arb_sepolia_client().await;

    let caller = wallet::get_wallet(Chain::ArbitrumSepolia);
    // println!("Caller Address: {caller.address()}");
    let client: Client = SignerMiddleware::new(arb_provider.clone(), caller.clone());
    call_function::erc_721_mint(client.clone(), U256::from(32)).await;

    // // test the name function
    // let name = arb_721_nft.name().call().await?;
    // println!("NFT name: {name}");

    // let tx = arb_721_nft.mint(caller.address(), U256::from(11)).send().await?.await?;
    // println!("Minted NFT with tx hash: {}", serde_json::to_string(&tx)?);

    Ok(())
}