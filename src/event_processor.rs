
use alloy::providers::RootProvider;
use alloy::pubsub::PubSubFrontend;
use alloy::{primitives::Log, rpc::types::eth::Block};
use anyhow::Result;
use crate::oracle::fetch_prices;
use crate::state::State;
use crate::interfaces::Morpho::*;
use std::sync::Arc;


pub fn process_new_block(block: Block, provider: Arc<RootProvider<PubSubFrontend>>, state: &mut State) {
        //let markets = state.get_all_markets();
        //let prices = fetch_prices(markets, provider);


        todo!()
}
