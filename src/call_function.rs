use std::{env, sync::Arc};
use ethers::{abi::Address, prelude::*};
use serde_json;
use dotenv::dotenv;
use hex;
use crate::{abi::NFTMatket, ONFT721NFT, abi::ONFT1155NFT};

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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub async fn erc_721_set_approval_for_all(client: Client, operator: Address, approved: bool) -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();
  let nft_contract_address = env::var("ERC721_CA")
    .expect("ERC721 NFT Contract Address must be set")
    .parse::<Address>()
    .unwrap();
  let contract = ONFT721NFT::new(nft_contract_address.clone(), Arc::new(client.clone()));
  let tx = contract.set_approval_for_all(operator, approved).send().await?.await?;
  println!("Set Approval For All with tx hash: {}", serde_json::to_string(&tx)?);
  Ok(())
}

#[allow(dead_code)]
pub async fn erc_721_list(client: Client, token_id: U256, price: U256) -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();
  let nft_contract_address = env::var("ERC721_CA")
    .expect("ERC721 NFT Contract Address must be set")
    .parse::<Address>()
    .unwrap();
  let nft_contract = ONFT721NFT::new(nft_contract_address.clone(), Arc::new(client.clone()));
  let market_contract_address = env::var("MARKET_CA")
    .expect("Marketplace Contract Address must be set")
    .parse::<Address>()
    .unwrap();
  let contract = NFTMatket::new(market_contract_address.clone(), Arc::new(client.clone()));
  // check if approved for all
  let approved = nft_contract.is_approved_for_all(client.address(), market_contract_address.clone()).call().await?;
  if !approved {
    let tx = nft_contract.set_approval_for_all(market_contract_address.clone(), true).send().await?.await?;
    println!("Set Approval For All with tx hash: {}", serde_json::to_string(&tx)?);
  }
  let tx = contract.list_nft(nft_contract_address, token_id, price).send().await?.await?;
  println!("Listed NFT with tx hash: {}", serde_json::to_string(&tx)?);
  Ok(())
}

#[allow(dead_code)]
pub async fn erc_1155_mint(client: Client, token_id: U256, amount: U256) -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();
  let nft_contract_address = env::var("ERC1155_CA")
    .expect("ERC1155 NFT Contract Address must be set")
    .parse::<Address>()
    .unwrap();
  let contract = ONFT1155NFT::new(nft_contract_address.clone(), Arc::new(client.clone()));
  let tx = contract.mint(client.address(), token_id, amount).send().await?.await?;
  println!("Minted NFT with tx hash: {}", serde_json::to_string(&tx)?);
  Ok(())
}

#[allow(dead_code)]
pub async fn erc_1155_cross_chain_send(client: Client, token_id: U256, amount: U256) -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();
  let nft_contract_address = env::var("ERC1155_CA")
    .expect("ERC1155 NFT Contract Address must be set")
    .parse::<Address>()
    .unwrap();
  let contract = ONFT1155NFT::new(nft_contract_address.clone(), Arc::new(client.clone()));
  // get estimated fees
  let fees = contract.estimate_send_fee(
    10232, 
    Bytes::from(client.address().to_fixed_bytes()), 
    token_id, 
    amount, 
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

#[allow(dead_code)]
pub async fn erc_1155_set_approval_for_all(client: Client, operator: Address, approved: bool) -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();
  let nft_contract_address = env::var("ERC1155_CA")
    .expect("ERC1155 NFT Contract Address must be set")
    .parse::<Address>()
    .unwrap();
  let contract = ONFT1155NFT::new(nft_contract_address.clone(), Arc::new(client.clone()));
  let tx = contract.set_approval_for_all(operator, approved).send().await?.await?;
  println!("Set Approval For All with tx hash: {}", serde_json::to_string(&tx)?);
  Ok(())
}

#[allow(dead_code)]
pub async fn erc_1155_list(client: Client, token_id: U256, amount: U256, price: U256) -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();
  let nft_contract_address = env::var("ERC1155_CA")
    .expect("ERC1155 NFT Contract Address must be set")
    .parse::<Address>()
    .unwrap();
  let nft_contract = ONFT1155NFT::new(nft_contract_address.clone(), Arc::new(client.clone()));
  let market_contract_address = env::var("MARKET_CA")
    .expect("Marketplace Contract Address must be set")
    .parse::<Address>()
    .unwrap();
  let contract = NFTMatket::new(market_contract_address.clone(), Arc::new(client.clone()));
  // check if approved for all
  let approved = nft_contract.is_approved_for_all(client.address(), market_contract_address.clone()).call().await?;
  if !approved {
    let tx = nft_contract.set_approval_for_all(market_contract_address.clone(), true).send().await?.await?;
    println!("Set Approval For All with tx hash: {}", serde_json::to_string(&tx)?);
  }
  let tx = contract.list_1155nft(nft_contract_address, token_id, amount, price).send().await?.await?;
  println!("Listed NFT with tx hash: {}", serde_json::to_string(&tx)?);
  Ok(())
}