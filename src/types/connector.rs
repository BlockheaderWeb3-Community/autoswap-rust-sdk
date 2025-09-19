use serde::Serialize;
use thiserror::Error;
/// Configuration for the AutoSwappr SDK=

#[derive(Debug, Serialize)]
pub struct AutoSwapprConfig {
    pub contract_address: String,
    pub rpc_url: String,
    pub account_address: String,
    pub private_key: String,
}

/// Ekubo pool key structure
#[derive(Debug, Serialize)]
pub struct PoolKey {
    pub token0: String,    // First token in the pool
    pub token1: String,    // Second token in the pool
    pub fee: u128,         // Pool fee in basis points (u128)
    pub tick_spacing: u32, // Pool extension parameter (felt252)
}

/// Amount to swap with magnitude and sign
#[derive(Debug, Serialize)]
pub struct Amount {
    pub mag: u128,  //Uint256
    pub sign: bool, // Always positive for swaps
}

/// Ekubo swap parameters
#[derive(Debug, Serialize)]
pub struct SwapParameters {
    pub amount: Amount,         // Amount to swap with magnitude and sign
    pub sqrt_ratio_limit: u128, // Price limit for the swap (u128)
    pub is_token1: bool,        // Whether the input token is token1
    pub skip_ahead: u32,        // Skip ahead parameter (u32)
}

/// Swap data structure for ekubo_manual_swap function
#[derive(Debug, Serialize)]
pub struct SwapData {
    pub params: SwapParameters,
    pub pool_key: PoolKey,
    pub caller: String,
}

#[allow(dead_code)]
/// Fee type enum
#[derive(Debug, Serialize)]
pub enum FeeType {
    Fixed,
    Percentage,
}

/// Contract information structure
#[derive(Debug, Serialize)]
pub struct ContractInfo {
    pub fees_collector: String,
    pub fibrous_exchange_address: String,
    pub avnu_exchange_address: String,
    pub oracle_address: String,
    pub owner: String,
    pub fee_type: FeeType,
    pub percentage_fee: u128,
}

/// Token information for supported tokens
#[derive(Debug, Serialize)]
pub struct TokenInfo {
    pub address: String,
    pub symbol: String,
    pub decimals: u32,
    pub name: String,
}

/// Pool configuration for different token pairs
#[derive(Debug, Serialize)]
pub struct PoolConfig {
    pub token0: String,
    pub token1: String,
    pub fee: u32,
    pub tick_spacing: String,
    pub extension: String,
    pub sqrt_ratio_limit: String,
}

///  Swap options for configuring the swap
#[derive(Debug, Serialize)]
pub struct SwapOptions {
    pub amount: String,           // Amount in wei (with decimals)
    pub is_token1: bool,          // Whether input token is token1 (defaults to false)
    pub skip_ahead: u32,          // Skip ahead parameter (defaults to 0)
    pub sqrt_ratio_limit: String, // Custom sqrt ratio limit
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AutoSwapprError {
    #[error("Insufficient allowance. Required: {required}, Available: {available}")]
    InsufficientAllowance { required: String, available: String },
    #[error("Unsupported token: {token}")]
    UnsupportedToken { token: String },
    #[error("Amount cannot be zero")]
    ZeroAmount,
    #[error("Invalid pool configuration: {reason}")]
    InvalidPoolConfig { reason: String },
    #[error("Insufficient balance. Required: {required}, Available: {available}")]
    InsufficientBalance { required: String, available: String },
    #[error("Swap failed: {reason}")]
    SwapFailed { reason: String },
    #[error("Invalid input: {details}")]
    InvalidInput { details: String },
    #[error("{message}")]
    Other { message: String },
}

#[allow(dead_code)]
impl FeeType {
    pub fn fee_type(&self) -> u8 {
        match self {
            FeeType::Fixed => 0,
            FeeType::Percentage => 1,
        }
    }
}

// Or create a constant that uses them
