use crate::models::network_model::Network;
use crate::models::wallet_model::Wallet;
use crate::services::db::Database;
use dotenv::dotenv;
use reqwest::Client;
use serde_json::json;
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use tokio::time::{sleep, Duration};

const BKNETWORK: Network = Network::Starknet;
const MAX_THREADS: u128 = 3;

pub async fn crawl_starknet(db: Database, interval: u64) {
    dotenv().ok();
    let rpc_url = env::var("RPC").expect("DATABASE_URL not found");

    loop {
        check_new_block(rpc_url.clone(), db.clone()).await;
        println!("Block checking completed... Going to sleep for {interval} seconds");
        sleep(Duration::from_secs(interval)).await;
        println!("Awake and scanning for new transactions...");
    }
}

pub async fn check_new_block(rpc_url: String, db: Database) {
    println!("Block checking started...");
    let last_scanned_block: u128 = db.get_last_scanned_block(BKNETWORK).await.unwrap_or(0);

    let latest_block = get_latest_block(rpc_url.clone()).await;

    if let Ok(block_number) = latest_block {
        if block_number > last_scanned_block {
            if block_number - last_scanned_block > 1 {
                let _result =
                    check_and_handle_skipped_bocks(last_scanned_block, block_number, rpc_url, db)
                        .await;
            } else {
                handle_new_block(block_number, rpc_url, db).await;
            }
        } else {
            println!("Already scanned block number: {}", block_number);
        }
    } else {
        println!("Failed to fetch latest starknet block");
    }
}

pub async fn handle_new_block(block_number: u128, rpc_url: String, db: Database) {
    let transactions = fetch_transactions(block_number, rpc_url.clone()).await;
    if let Ok(transactions) = transactions {
        println!("New transactions detected");
        process_transactions(transactions, db.clone()).await;
        let mut _network_data = db.update_last_scanned_block(BKNETWORK, block_number).await;
    } else {
        println!("Failed to fetch transactions from block");
    }
}

pub async fn handle_new_block_arc(block_number: u128, rpc_url: Arc<String>, db: Arc<Database>) {
    let rpc_url_str = rpc_url.to_string();
    let db_str = (*db).clone();
    handle_new_block(block_number, rpc_url_str, db_str).await;
}

pub async fn get_latest_block(rpc_url: String) -> Result<u128, String> {
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
        let block_number: u128 = block.as_u64().unwrap().into();
        return Ok(block_number);
        // Log the result
    } else {
        println!("Error occurred: {:?}", response.status());
        return Err("Failed to fetch latest starknet block".to_string());
    }
}

pub async fn fetch_transactions(block_number: u128, rpc_url: String) -> Result<String, String> {
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

pub async fn process_transactions(transactions: String, db: Database) {
    let wallets: Vec<Wallet> = match db.get_all_wallets_via_network(BKNETWORK).await {
        Ok(wallets) => wallets,
        Err(err) => {
            println!("Failed to get wallets: {:?}", err);
            return;
        }
    };

    println!("LOG:: Searching for transactions from registered wallets...");

    let transactions_json: serde_json::Value = serde_json::from_str(&transactions).unwrap();

    let same_network_wallets = filter_networks(wallets, BKNETWORK);

    // print_addresses(transactions_json.clone(), same_network_wallets.clone());

    let relevant_tx: Vec<&serde_json::Value> = transactions_json
        .as_array()
        .expect("transactions is not json")
        .into_iter()
        .filter(|tx| {
            same_network_wallets.contains(&format!(
                "0x0{}",
                tx["sender_address"]
                    .as_str()
                    .unwrap_or("0x000000000000000000000000")[2..]
                    .to_string()
            ))
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

pub fn print_addresses(transactions_json: serde_json::Value, same_network_wallets: Vec<String>) {
    let transaction_senders: Vec<String> = transactions_json
        .as_array()
        .expect("transactions_json is not an array")
        .iter()
        .map(|tx| {
            format!(
                "0x0{}",
                tx["sender_address"]
                    .as_str()
                    .unwrap_or("0x000000000000000000000000")[2..]
                    .to_string()
            )
        })
        .collect();

    println!(
        "ADDRESSES THAT TRIGGERED TRANSACTIONS: {:?}",
        transaction_senders
    );
    println!("ADDRESSES IN DATABASE: {:?}", same_network_wallets);
}

pub async fn check_and_handle_skipped_bocks(
    last_scanned_block: u128,
    current_block: u128,
    rpc_url: String,
    db: Database,
) -> Result<(), Box<dyn Error>> {
    let skipped_blocks = current_block.saturating_sub(last_scanned_block);
    println!("BLOCKS SKIPPED: {:?}", skipped_blocks);

    if skipped_blocks <= 3 {
        for block_number in (last_scanned_block + 1)..=current_block {
            handle_new_block(block_number, rpc_url.clone(), db.clone()).await;
        }
        Ok(())
    } else {
        let rpc_url = Arc::new(rpc_url);
        let db = Arc::new(db);
        // For more than 5 skipped blocks, create a thread pool of up to 3 threads
        let blocks_per_thread: u128 = ((skipped_blocks as f64 / MAX_THREADS as f64).ceil() as u64)
            .max(1)
            .into();
        let thread_pool = Arc::new(Mutex::new(Vec::new()));

        for thread_index in 0..MAX_THREADS {
            let thread_pool = Arc::clone(&thread_pool);
            let start_block = last_scanned_block + 1 + thread_index * blocks_per_thread;
            let end_block = (start_block + blocks_per_thread - 1).min(current_block);
            let rpc_url = Arc::clone(&rpc_url);
            let db = Arc::clone(&db);

            if start_block > current_block {
                break;
            }

            task::spawn(async move {
                let mut thread = thread_pool.lock().await;
                thread.push(thread_index);

                for block_number in start_block..=end_block {
                    handle_new_block_arc(block_number, rpc_url.clone(), db.clone()).await;
                }
                println!(
                    "Thread {} completed processing from {} to {}",
                    thread_index, start_block, end_block
                );
            })
            .await?;
        }
        Ok(())
    }
}
