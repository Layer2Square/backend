use std::{env, sync::Arc};
use ethers::prelude::*;
use dotenv::dotenv;
use crate::{ONFT721NFT, ONFT1155NFT};

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

pub async fn erc_721_mint(client: Client, token_id: U256) -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();
  let nft_contract_address = env::var("ERC721_CA")
    .expect("ERC721 NFT Contract Address must be set")
    .parse::<Address>()
    .unwrap();
  let contract = ONFT721NFT::new(nft_contract_address.clone(), Arc::new(client.clone()));
  let tx = contract.mint(client.address(), token_id).send().await?.await?;
  println!("Minted NFT with tx hash: {}", serde_json::to_string(&tx)?);
  Ok(())
}