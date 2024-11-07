use crate::models::network_model::Network;
use crate::models::wallet_model::Wallet;
use dotenv::dotenv;
use reqwest::Client;
use serde_json::json;
use std::env;
use tokio::time::{sleep, Duration};

pub async fn crawl_starknet(interval: u64) {
    let mut last_scanned_block: u64 = 0;
    dotenv().ok();
    let rpc_url = env::var("RPC").expect("DATABASE_URL not found");

    loop {
        check_new_block(rpc_url.clone(), &mut last_scanned_block).await;
        println!("Block checking completed... Going to sleep for {interval} seconds");
        sleep(Duration::from_secs(interval)).await;
        println!("Awake and scanning for new transactions...");
    }
}

pub async fn check_new_block(rpc_url: String, last_scanned_block: &mut u64) {
    println!("Block checking started...");
    let latest_block = get_latest_block(rpc_url.clone()).await;
    if let Ok(block_number) = latest_block {
        if block_number > *last_scanned_block {
            *last_scanned_block = block_number.clone();
            let transactions = fetch_transactions(block_number, rpc_url.clone()).await;
            if let Ok(transactions) = transactions {
                println!("New transactions detected");
                process_transactions(transactions);
            } else {
                println!("Failed to fetch transactions from block");
            }
        } else {
            println!("Already scanned block number: {}", block_number);
        }
    } else {
        println!("Failed to fetch latest starknet block");
    }
}

pub async fn get_latest_block(rpc_url: String) -> Result<u64, String> {
    // Create a JSON-RPC request body
    let request_body = json!({
        "jsonrpc": "2.0",
        "method": "starknet_blockNumber",
        "params": [],
        "id": 0
    });

    // Log message
    println!("LOG:: Fetching latest starknet block");

    // Send the POST request
    let client = Client::new();
    let response = client
        .post(rpc_url)
        .json(&request_body)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();

    // Check if the request was successful
    if response.status().is_success() {
        let response_json: serde_json::Value = response.json().await.unwrap();
        let block: &serde_json::Value = &response_json["result"];
        println!("LOG: Block number query response data: {:?}", block);
        let block_number: u64 = block.as_u64().unwrap();
        return Ok(block_number);
        // Log the result
    } else {
        println!("Error occurred: {:?}", response.status());
        return Err("Failed to fetch latest starknet block".to_string());
    }
}

pub async fn fetch_transactions(block_number: u64, rpc_url: String) -> Result<String, String> {
    println!("LOG:: Fetched transaction for block number: {block_number}");

    // Create a JSON-RPC request body
    let request_body = json!({
        "jsonrpc": "2.0",
        "method": "starknet_getBlockWithTxs",
        "params": {
            "block_id": {
                "block_number": block_number,

            }
        },
        "id": 0
    });

    // Send the POST request
    let client = Client::new();
    let response = client
        .post(rpc_url)
        .json(&request_body)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();

    // Check if the request was successful
    if response.status().is_success() {
        let response_json: serde_json::Value = response.json().await.unwrap();
        println!("{}", response_json.is_object());

        let transactions = &response_json["result"]["transactions"];
        return Ok(transactions.to_string());
        // Log the result
    } else {
        println!("Error occurred: {:?}", response.status());
        return Err("Failed to fetch latest starknet block".to_string());
    }
}

pub fn process_transactions(transactions: String) {
    let wallet1 = Wallet::new(
        "0x362abd06d55f90e9df52eecdf0bbf15e8f49b33371b19a7b120d911dfcb39e".to_string(),
        "starknet".to_string(),
    )
    .unwrap();
    let wallet2 = Wallet::new(
        "0x449986bdc95c8b602bf3726d9655abd498f43540449be398ee97922e1c80b30".to_string(),
        "starknet".to_string(),
    )
    .unwrap();
    let wallet3 = Wallet::new(
        "0x325200068f8e7f3b0d37233a0583419f898386b24b72c89854a099ca3847089".to_string(),
        "starknet".to_string(),
    )
    .unwrap();

    let wallets: Vec<Wallet> = vec![wallet1, wallet2, wallet3];

    println!("LOG:: Searching for transactions from registered wallets...");

    let transactions_json: serde_json::Value = serde_json::from_str(&transactions).unwrap();

    let same_network_wallets = filter_networks(wallets, Network::Starknet);

    let relevant_tx: Vec<&serde_json::Value> = transactions_json
        .as_array()
        .expect("transactions is not json")
        .into_iter()
        .filter(|tx| {
            same_network_wallets.contains(
                &tx["sender_address"]
                    .as_str()
                    .unwrap_or("0x000000000000000000000000")
                    .to_string(),
            )
        })
        .collect();

    if relevant_tx.is_empty() {
        println!("No relevant transactions found.");
        return;
    } else {
        println!("LOG:: {}, Relevant transactions found:", relevant_tx.len());
    }
}

pub fn filter_networks(wallets: Vec<Wallet>, network: Network) -> Vec<String> {
    let mut same_network_wallets: Vec<String> = Vec::new();
    for wallet in wallets {
        if wallet.network == network {
            same_network_wallets.push(wallet.wallet_address);
        }
    }
    same_network_wallets
}
