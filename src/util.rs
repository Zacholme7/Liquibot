
use alloy::providers::{Provider, RootProvider};
use alloy::sol_types::{SolEvent, SolEventInterface};
use std::sync::Arc;
use alloy::pubsub::PubSubFrontend;
use alloy::rpc::types::eth::{Filter, Log};
use anyhow::Result;
use crate::interfaces::Morpho::{self, MorphoEvents};
use crate::state::State;

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


