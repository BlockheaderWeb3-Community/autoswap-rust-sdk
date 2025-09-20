pub mod client;
pub mod constant;
pub mod contracts;
pub mod provider;
pub mod simple_client;
pub mod types;

// Re-export main types and clients for easy access
pub use client::AutoSwapprClient;
pub use contracts::{AutoSwapprContract, Erc20Contract, addresses};
pub use provider::{Network, StarknetProvider};
pub use simple_client::{
    SimpleAutoSwapprClient, SimpleConfig, SimpleError, SwapData as SimpleSwapData,
};
pub use types::connector::{
    Amount, AutoSwapprConfig, AutoSwapprError, ContractInfo, Delta, FeeType, I129, PoolKey, Route,
    RouteParams, SwapData, SwapOptions, SwapParameters, SwapParams, SwapResult, TokenInfo, Uint256,
};
