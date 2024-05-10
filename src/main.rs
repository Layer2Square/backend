use ethers::prelude::*;

mod client;
mod call_function;
mod wallet;

mod abi;
use abi::ONFT721NFT;

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = client::get_sepolia_client().await;
    let block_number: U64 = provider.get_block_number().await?;
    println!("Current Block Number is {block_number}");

    let arb_provider = client::get_arb_sepolia_client().await;

    let caller = wallet::get_wallet(Chain::ArbitrumSepolia);
    // println!("Caller Address: {caller.address()}");
    let client: Client = SignerMiddleware::new(arb_provider.clone(), caller.clone());
    // call_function::erc_721_mint(client.clone(), U256::from(32)).await;
    let _ = call_function::erc_721_cross_chain_send(client.clone(), U256::from(11)).await;

    Ok(())
}