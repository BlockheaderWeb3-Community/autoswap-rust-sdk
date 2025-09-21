use serde::{Deserialize, Serialize};
use thiserror::Error;
/// Configuration for the AutoSwappr SDK
#[derive(Debug, Serialize, Deserialize)]
pub struct AutoSwapprConfig {
    pub contract_address: String,
    pub rpc_url: String,
    pub account_address: String,
    pub private_key: String,
}

/// Uint256 representation for Starknet
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Uint256 {
    pub low: u128,
    pub high: u128,
}

impl Uint256 {
    pub fn from_u128(value: u128) -> Self {
        Self {
            low: value,
            high: 0,
        }
    }

    pub fn from_string(value: &str) -> Result<Self, String> {
        let parsed = value.parse::<u128>().map_err(|_| "Invalid number format")?;
        Ok(Self::from_u128(parsed))
    }

    pub fn to_hex_string(&self) -> String {
        // Convert Uint256 to hex string
        format!("0x{:032x}{:032x}", self.high, self.low)
    }
}

/// Ekubo pool key structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoolKey {
    pub token0: String,    // First token in the pool
    pub token1: String,    // Second token in the pool
    pub fee: u128,         // Pool fee in basis points (u128)
    pub tick_spacing: u32, // Pool extension parameter (felt252)
    pub extension: String, // Pool extension parameter
}

/// Amount to swap with magnitude and sign
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Amount {
    pub mag: Uint256, // Uint256 magnitude
    pub sign: bool,   // Always positive for swaps
}

/// Ekubo swap parameters
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SwapParameters {
    pub amount: Amount,            // Amount to swap with magnitude and sign
    pub sqrt_ratio_limit: Uint256, // Price limit for the swap (Uint256)
    pub is_token1: bool,           // Whether the input token is token1
    pub skip_ahead: u32,           // Skip ahead parameter (u32)
}

/// Swap data structure for ekubo_manual_swap function
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SwapData {
    pub params: SwapParameters,
    pub pool_key: PoolKey,
    pub caller: String,
}

/// Route structure for AVNU swaps
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Route {
    pub token_from: String,
    pub token_to: String,
    pub exchange_address: String,
    pub percent: u128,
    pub additional_swap_params: Vec<String>,
}

/// Route parameters for Fibrous swaps
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteParams {
    pub token_in: String,
    pub token_out: String,
    pub amount_in: Uint256,
    pub min_received: Uint256,
    pub destination: String,
}

/// Swap parameters for Fibrous swaps
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SwapParams {
    pub token_in: String,
    pub token_out: String,
    pub rate: u32,
    pub protocol_id: u32,
    pub pool_address: String,
    pub extra_data: Vec<String>,
}

/// Swap result structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SwapResult {
    pub delta: Delta,
}

/// Delta structure for swap results
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Delta {
    pub amount0: I129,
    pub amount1: I129,
}

/// I129 structure for Ekubo amounts
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct I129 {
    pub mag: u128,
    pub sign: bool,
}

/// Fee type enum
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FeeType {
    Fixed,
    Percentage,
}

impl FeeType {
    pub fn to_u8(&self) -> u8 {
        match self {
            FeeType::Fixed => 0,
            FeeType::Percentage => 1,
        }
    }

    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => FeeType::Fixed,
            _ => FeeType::Percentage,
        }
    }
}

/// Contract information structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractInfo {
    pub fees_collector: String,
    pub fibrous_exchange_address: String,
    pub avnu_exchange_address: String,
    pub oracle_address: String,
    pub owner: String,
    pub fee_type: FeeType,
    pub percentage_fee: u16,
}

/// Token information for supported tokens
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenInfo {
    pub address: String,
    pub symbol: String,
    pub decimals: u8,
    pub name: String,
}

/// Pool configuration for different token pairs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoolConfig {
    pub token0: String,
    pub token1: String,
    pub fee: u128,
    pub tick_spacing: u32,
    pub extension: String,
    pub sqrt_ratio_limit: String,
}

/// Swap options for configuring the swap
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SwapOptions {
    pub amount: String,                   // Amount in wei (with decimals)
    pub is_token1: Option<bool>,          // Whether input token is token1 (defaults to false)
    pub skip_ahead: Option<u32>,          // Skip ahead parameter (defaults to 0)
    pub sqrt_ratio_limit: Option<String>, // Custom sqrt ratio limit
}

/// Error types for the AutoSwappr SDK
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
    #[error("Network error: {message}")]
    NetworkError { message: String },
    #[error("Contract error: {message}")]
    ContractError { message: String },
    #[error("Provider error: {message}")]
    ProviderError { message: String },
    #[error("{message}")]
    Other { message: String },
}
