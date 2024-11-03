// src/main.rs
use warp::Filter;
use ethers::prelude::*;
use std::convert::Infallible;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Connect to Ethereum via an Infura or Alchemy provider (replace with your own API key)
    let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/YOUR_INFURA_API_KEY")
        .expect("Invalid provider URL");
    let provider = Arc::new(provider);

    // Route to get balance of an Ethereum address
    let provider_clone = provider.clone();
    let balance_route = warp::path!("balance" / String)
        .and(warp::any().map(move || provider_clone.clone()))
        .and_then(handle_get_balance);

    // Route to get block information
    let provider_clone = provider.clone();
    let block_route = warp::path!("block" / u64)
        .and(warp::any().map(move || provider_clone.clone()))
        .and_then(handle_get_block);

    // Route to send a transaction
    let provider_clone = provider.clone();
    let send_tx_route = warp::path!("send-transaction")
        .and(warp::body::json())
        .and(warp::any().map(move || provider_clone.clone()))
        .and_then(handle_send_transaction);

    let routes = balance_route.or(block_route).or(send_tx_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_get_balance(address: String, provider: Arc<Provider<Http>>) -> Result<impl warp::Reply, Infallible> {
    let address: Address = address.parse().expect("Invalid address format");
    match provider.get_balance(address, None).await {
        Ok(balance) => Ok(warp::reply::json(&format!("Balance: {}", balance))),
        Err(_) => Ok(warp::reply::json(&"Failed to fetch balance")),
    }
}

async fn handle_get_block(block_number: u64, provider: Arc<Provider<Http>>) -> Result<impl warp::Reply, Infallible> {
    match provider.get_block(block_number).await {
        Ok(Some(block)) => Ok(warp::reply::json(&block)),
        Ok(None) => Ok(warp::reply::json(&"Block not found")),
        Err(_) => Ok(warp::reply::json(&"Failed to fetch block")),
    }
}

async fn handle_send_transaction(
    body: serde_json::Value,
    provider: Arc<Provider<Http>>
) -> Result<impl warp::Reply, Infallible> {
    // Extract sender, receiver, and amount from the JSON body
    let sender: Address = body["sender"].as_str().expect("Missing sender").parse().expect("Invalid sender address");
    let receiver: Address = body["receiver"].as_str().expect("Missing receiver").parse().expect("Invalid receiver address");
    let amount: U256 = U256::from_dec_str(body["amount"].as_str().expect("Missing amount")).expect("Invalid amount");

    // Configure the wallet and sign the transaction
    let wallet = Wallet::from("YOUR_PRIVATE_KEY").connect(provider.clone());
    let tx = TransactionRequest::new()
        .from(sender)
        .to(receiver)
        .value(amount);

    match wallet.send_transaction(tx, None).await {
        Ok(pending_tx) => Ok(warp::reply::json(&format!("Transaction sent: {:?}", pending_tx.tx_hash()))),
        Err(_) => Ok(warp::reply::json(&"Failed to send transaction")),
    }
}
