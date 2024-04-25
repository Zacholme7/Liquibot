use alloy::sol;
use alloy::sol_types::{SolEvent, SolEventInterface};

sol!(
        Liquidator,
        "contracts/out/Liquidator.sol/Liquidator.json"
    );
sol!(
        #[derive(Debug)]
        Morpho,
        "contracts/out/IMorpho.sol/IMorpho.json"
);
sol!(
        #[derive(Debug)]
        #[sol(rpc)]
        Oracle,
        "contracts/out/IOracle.sol/IOracle.json"
);