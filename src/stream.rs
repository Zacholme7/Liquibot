
use alloy::{providers::{Provider, RootProvider}, pubsub::PubSubFrontend, sol_types::SolEvent};
use alloy::rpc::types::eth::{Filter};
use std::sync::Arc;
use anyhow::Result;
use futures_util::stream::StreamExt;
use crate::util::Morpho::{self, MorphoEvents};
use crate::event_processor::*;
use crate::state::State;


/// Stream events from new blocks
pub async fn stream(ws: Arc<RootProvider<PubSubFrontend>>, state: &mut State) -> Result<()> {
        // Set up the log subscription filter and stream
        let filter = Filter::new().event(Morpho::CreateMarket::SIGNATURE);
        let sub = ws.subscribe_logs(&filter).await?;
        let mut stream = sub.into_stream();

        // Set up the block subscription stream
        let block_sub = ws.subscribe_blocks().await?;
        let mut block_stream = block_sub.into_stream();

        // handle both streams
        loop {
                tokio::select! {
                        Some(log) = stream.next() => MorphoEvents::process_log(log, state)?,
                        Some(block) = block_stream.next() => process_new_block(block) 
                }
        }

        Ok(())
}