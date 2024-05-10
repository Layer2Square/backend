use std::{env, sync::Arc};
use ethers::{abi::Address, prelude::*};
use serde_json;
use dotenv::dotenv;
use hex;
use crate::ONFT721NFT;

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

#[allow(dead_code)]
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

pub async fn erc_721_cross_chain_send(client: Client, token_id: U256) -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();
  let nft_contract_address = env::var("ERC721_CA")
    .expect("ERC721 NFT Contract Address must be set")
    .parse::<Address>()
    .unwrap();
  let contract = ONFT721NFT::new(nft_contract_address.clone(), Arc::new(client.clone()));
  // get estimated fees
  let fees = contract.estimate_send_fee(
    10232, 
    Bytes::from(client.address().to_fixed_bytes()), 
    token_id, 
    false, 
    Bytes::from(hex::decode("00010000000000000000000000000000000000000000000000000000000000030d40").unwrap())
  ).call()
  .await?;
  println!("fee: {}", serde_json::to_string(&fees.0)?);
  let tx = contract.send_from(
    client.address(), 
    10232, 
    Bytes::from(client.address().to_fixed_bytes()), 
    token_id, 
    client.address(), 
    Address::zero(), 
    Bytes::from(hex::decode("00010000000000000000000000000000000000000000000000000000000000030d40").unwrap())
  )
  .value(fees.0)
  .send().await.unwrap().await.unwrap();
  println!("Sent NFT with tx hash: {}", serde_json::to_string(&tx)?);
  Ok(())
}