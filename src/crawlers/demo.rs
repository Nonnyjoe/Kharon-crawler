use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;

async fn get_last_scanned_block() -> u64 {
    // Mock function for retrieving the last scanned block from the database
    100
}

async fn get_latest_block() -> u64 {
    // Mock function for retrieving the latest block number from the RPC
    150
}

async fn process_block(block_number: u64) {
    println!("Processing block {}", block_number);
    // Replace with actual processing logic, e.g., fetching and handling block data
}

async fn process_skipped_blocks() -> Result<(), Box<dyn Error>> {
    let last_scanned_block = get_last_scanned_block().await;
    let latest_block = get_latest_block().await;

    let skipped_blocks = latest_block.saturating_sub(last_scanned_block);
    println!("Skipped blocks: {}", skipped_blocks);

    // Set the maximum number of threads
    let max_threads = 3;

    // If there are fewer than or equal to 5 skipped blocks, process sequentially
    if skipped_blocks <= 5 {
        for block_number in (last_scanned_block + 1)..=latest_block {
            process_block(block_number).await;
        }
    } else {
        // For more than 5 skipped blocks, create a thread pool of up to 3 threads
        let blocks_per_thread = ((skipped_blocks as f64 / max_threads as f64).ceil() as u64).max(1);
        let thread_pool = Arc::new(Mutex::new(Vec::new()));

        for thread_index in 0..max_threads {
            let thread_pool = Arc::clone(&thread_pool);
            let start_block = last_scanned_block + 1 + thread_index * blocks_per_thread;
            let end_block = (start_block + blocks_per_thread - 1).min(latest_block);

            if start_block > latest_block {
                break;
            }

            task::spawn(async move {
                let mut thread = thread_pool.lock().await;
                thread.push(thread_index);

                for block_number in start_block..=end_block {
                    process_block(block_number).await;
                }

                println!(
                    "Thread {} completed processing from {} to {}",
                    thread_index, start_block, end_block
                );
            })
            .await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = process_skipped_blocks().await {
        eprintln!("Error: {}", e);
    }
}
