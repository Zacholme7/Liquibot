use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::client::WsConnect;
use alloy::sol_types::SolValue;
use anyhow::Result;
use dotenv;
use std::sync::Arc;

use liquibot::util::sync_to_latest_block;


#[tokio::main]
async fn main() -> Result<()> {
    // load the dot env
    dotenv::dotenv().ok();

    // construct http provider
    let http_url = std::env::var("HTTP_RPC_URL")?.parse()?;
    let http = Arc::new(ProviderBuilder::new().on_http(http_url)?);

    // construct ws provider
    let wss_url = std::env::var("WSS_RPC_URL")?;
    let ws_conn = WsConnect::new(wss_url);
    let ws = Arc::new(ProviderBuilder::new().on_ws(ws_conn).await?);

    sync_to_latest_block(ws.clone()).await?;
    // make the hash of it


    println!("Hello, world!");
    Ok(())
}
