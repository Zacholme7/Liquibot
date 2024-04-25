
use std::sync::Arc;
use alloy::providers::{Provider, RootProvider};
use alloy::sol_types::{SolEvent, SolEventInterface};
use alloy::transports::http::Http;
use alloy::transports::BoxTransport;
use alloy::pubsub::PubSubFrontend;
use alloy::rpc::types::eth::{Filter, Log};
use alloy::primitives::Address;
use anyhow::Result;
use alloy::sol;
use crate::util::Morpho::MorphoEvents;
use crate::state::State;

const MORPHO_ADDRESS: &str = "0xBBBBBbbBBb9cC5e90e3b3Af64bdAF62C37EEFFCb";
sol!(
    Liquidator,
    "contracts/out/Liquidator.sol/Liquidator.json"

);
sol!(
    #[derive(Debug)]
    Morpho,
    "contracts/out/IMorpho.sol/IMorpho.json"
);

pub async fn sync_to_latest_block(provider: Arc<RootProvider<PubSubFrontend>>, state: &mut State) -> Result<()> {
    // get the current block 
    let latest_block = provider.get_block_number().await?;
    //let mut logs = Vec::new();

    let mut blocks_processed: u64 = 0;
    let mut block_range = Vec::new();

    let block_from = 15_000_000;
    let block_to = latest_block;

    // construct all the block intervals
    loop {
        let interval_start_block = block_from + blocks_processed;
        let mut interval_end_block = interval_start_block + 50_000 - 1;
        if interval_end_block > block_to {
            interval_end_block = block_to;
            block_range.push((interval_start_block, interval_end_block));
            break;
        }
        block_range.push((interval_start_block, interval_end_block));
        blocks_processed += 50_000;
    }
    // fetch all of the logs
    for interval in block_range {
        // construct a filter and fetch the logs
        let filter = Filter::new()
            .select(interval.0..interval.1)
            .event(Morpho::CreateMarket::SIGNATURE);
        let logs = provider.get_logs(&filter).await?;

        // process all of the logs
        for log in logs {
                MorphoEvents::process_log(log, state)?;
        }
    }
    Ok(())
}


