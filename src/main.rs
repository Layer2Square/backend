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

    #[allow(unused_variables)]
    let arb_provider = client::get_arb_sepolia_client().await;
    #[allow(unused_variables)]
    let op_provider = client::get_op_sepolia_client().await;
    #[allow(unused_variables)]
    let base_provider = client::get_base_sepolia_client().await;
    #[allow(unused_variables)]
    let zksync_provider = client::get_zksync_goerli_client().await;
    #[allow(unused_variables)]
    let zkevm_provider = client::get_zkevm_sepolia_client().await;
    #[allow(unused_variables)]
    let scroll_provider = client::get_scroll_sepolia_client().await;
    #[allow(unused_variables)]
    let linea_client = client::get_linea_testnet_client().await;
    #[allow(unused_variables)]
    let caller = wallet::get_wallet(Chain::OptimismSepolia);
    // println!("Caller Address: {caller.address()}");
    let client: Client = SignerMiddleware::new(op_provider.clone(), caller.clone());

    // ERC 721 test
    for i in 0..100 {
        // // test erc 721 function
        // let mint_used_ether = call_function::erc_721_mint(client.clone(), U256::from(i)).await.unwrap();
        // // print!("Minted NFT used ether: {:?}", mint_used_ether);
        // let list_use_ether = call_function::erc_721_list(client.clone(), U256::from(i), U256::from(100)).await.unwrap();
        // // print!("List NFT used ether: {:?}", list_use_ether);
        // let fees  = call_function::erc_721_cross_chain_send(client.clone(), U256::from(i)).await.unwrap();
        // // print!("Cross chain send NFT used ether fee: {:?}", fees);
        // // break;
        // println!("{:?},{:?},{:?}", mint_used_ether, list_use_ether, fees);

        // test erc 1155 function
        let mint_used_ether = call_function::erc_1155_mint(client.clone(), U256::from(i), U256::from(1)).await.unwrap();
        let list_use_ether = call_function::erc_1155_list(client.clone(), U256::from(i), U256::from(1), U256::from(100)).await.unwrap();
        let fees  = call_function::erc_1155_cross_chain_send(client.clone(), U256::from(i), U256::from(1)).await.unwrap();
        println!("{:?},{:?},{:?}", mint_used_ether, list_use_ether, fees);
    }

    Ok(())
}