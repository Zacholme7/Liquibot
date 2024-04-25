
use std::collections::HashMap;
use alloy::{primitives::{address, Address, U256}, providers::RootProvider, pubsub::PubSubFrontend};
use anyhow::Result;
use std::sync::Arc;
use ethers::{
        contract::{
            multicall_contract::{Aggregate3Call, Call3},
            MulticallContract, MULTICALL_ADDRESS,
        },
        providers::{Http, Provider, ProviderExt},
    };
use crate::interfaces::{Morpho::MarketParams, Oracle};

pub async fn fetch_prices(provider: &Arc<RootProvider<PubSubFrontend>>, markets: Vec<MarketParams>) ->  Result<()>{
        // mapping from oracle to the price
        let mut prices: HashMap<Address, U256> = HashMap::new();

        // make our provider.... need an alloy multicall
        let ether_provider = Arc::new(Provider::<Http>::try_connect(&std::env::var("HTTP_RPC_URL").unwrap()).await?);

        let multicall = MulticallContract::new(MULTICALL_ADDRESS, ether_provider);
        let mut multicall_calls: Aggregate3Call = Aggregate3Call { calls: vec![] };

        for params in markets {
                let oracle = Oracle::new(params.oracle, provider);
                let price_calldata = oracle.price().calldata();

                multicall_calls.calls.push(Call3 {
                        target: params.oracle,
                        call_data: price_calldata.to_owned(),
                        allow_failure: true,
                });

        }

        let results = multicall.aggregate_3(multicall_calls).call().await?;

        for (i, value) in results.iter().enumerate() {
                if !value.success {
                        continue;
                }
                let values = &values.return_data.0;
                prices.insert(params.oracle, decode(values));
        }

        Ok(())
}