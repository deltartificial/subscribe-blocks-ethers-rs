// Subscribe to new blocks with ethers-rs
// @author - deltartificial

use ethers::{prelude::*};
use eyre::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let client =
        Provider::<Ws>::connect("wss://YOUR-RPC-ENDPOINT")
            .await?;
    let client = Arc::new(client);

    let mut stream = client.watch_blocks().await?;
    while let Some(block) = stream.next().await {
        let block = client.get_block(block).await?.unwrap();
        println!(
            "-> Timestamp: {:?}, Block Number: {}, Block Hash: {:?}",
            block.timestamp,
            block.number.unwrap(),
            block.hash.unwrap()
        );
    }

    Ok(())
}