use std::collections::HashMap;
use alloy::primitives::{Address, U256};
use alloy::rpc::types::eth::Log;
use alloy::sol_types::SolEventInterface;
use crate::util::Morpho::{Market, MarketParams, Position};
use crate::util::Morpho::MorphoEvents;
use anyhow::Result;

impl MorphoEvents {
        pub fn process_log(log: Log, state: &mut State) -> Result<()> {
                let decoded = MorphoEvents::decode_log(&log.inner, true)?.data;
                match decoded {
                        MorphoEvents::CreateMarket(market) => {
                                state.add_market(market.id.into(), market.marketParams);
                        }
                        _ => {}
                }
                Ok(())
        }
}


#[derive(Debug, Default)]
pub struct State {
    pub last_block_sync: u64,
    /// maps the identifier to the market
    pub market: HashMap<U256, Market>,
    ///  maps the id to the market parameters
    pub market_config: HashMap<U256, MarketParams>,
    /// maps the idetifier to all positions
    pub market_positions: HashMap<U256, HashMap<Address, Position>>,
}

impl State {
        /// Add a new market to our state
        pub fn add_market(&mut self, id: U256, market_params: MarketParams) {
                self.market_config.insert(id, market_params);
                self.market_positions.insert(id, HashMap::new());
        }
}

