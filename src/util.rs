
use std::sync::Arc;
use alloy::providers::{Provider, RootProvider};
use alloy::sol_types::SolEventInterface;
use alloy::transports::http::Http;
use alloy::transports::BoxTransport;
use alloy::pubsub::PubSubFrontend;
use alloy::rpc::types::eth::{Filter, Log};
use alloy::primitives::Address;

//use alloy::rpc::types::eth::Log;
use anyhow::Result;
use alloy::sol;

use crate::util::Morpho::MorphoEvents;

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

pub async fn sync_to_latest_block(provider: Arc<RootProvider<PubSubFrontend>>) -> Result<()> {
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
            .address(MORPHO_ADDRESS.parse::<Address>().unwrap());
        let mut logs = provider.get_logs(&filter).await?;
        for log in logs {
            process_log(&log).await?;
            //logs.push(log);
        }
    }

    Ok(())
}


async fn process_log(log: &Log) -> Result<()> {
    let decoded = MorphoEvents::decode_log(&log.inner, true)?.data;
    match decoded {
        MorphoEvents::CreateMarket(market) => todo!(),
        MorphoEvents::Borrow(borrow) => todo!(),
        MorphoEvents::SupplyCollateral(supply_collateral) => todo!(),
        MorphoEvents::Repay(repay) => todo!(),
        MorphoEvents::WithdrawCollateral(withdraw_collaterla) => todo!(),
        MorphoEvents::Liquidate(liquidate) => todo!(),
        MorphoEvents::Supply(supply) => todo!(),
        MorphoEvents::Withdraw(withdraw) => todo!(),
        MorphoEvents::AccrueInterest(accrue_interest) => todo!(),
        _ => {}
    }
    Ok(())
}




