pub mod constant;
pub mod swappr;
pub mod types;

// Re-export main types and clients for easy access
pub use types::connector::{
    AutoSwappr, AutoSwapprError, ContractInfo, Delta, FeeType, I129, PoolKey, Route, SwapData,
    SwapOptions, SwapParameters, SwapParams, SwapResult,
};

pub use constant::{ETH, STRK, TokenAddress, TokenInfo, USDC, USDT, WBTC};

#[cfg(test)]
#[path = "contracts_test.rs"]
mod contracts_tests;
