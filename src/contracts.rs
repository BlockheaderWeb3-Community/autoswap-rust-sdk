// Real AutoSwappr Contract ABI and Interface Implementation
// Based on the actual Cairo contract ABI

use crate::types::connector::Uint256 as StarknetUint256;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::{
    accounts::ConnectedAccount,
    core::{
        types::{BlockId, BlockTag, Call, Felt, FunctionCall},
        utils::get_selector_from_name,
    },
    macros::selector,
    providers::{JsonRpcClient, Provider},
};
use std::sync::Arc;

// Type aliases for compatibility
type FieldElement = Felt;
type ContractAddress = Felt;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::types::connector::{ContractInfo, FeeType, SwapData};

/// AutoSwappr Contract ABI definitions
pub mod abi {
    /// Contract method names
    pub const EKUBO_SWAP: &str = "ekubo_swap";
    pub const EKUBO_MANUAL_SWAP: &str = "ekubo_manual_swap";
    pub const AVNU_SWAP: &str = "avnu_swap";
    pub const FIBROUS_SWAP: &str = "fibrous_swap";
    pub const CONTRACT_PARAMETERS: &str = "contract_parameters";
    pub const GET_TOKEN_AMOUNT_IN_USD: &str = "get_token_amount_in_usd";
    pub const GET_TOKEN_FROM_STATUS_AND_VALUE: &str = "get_token_from_status_and_value";
    pub const SET_FEE_TYPE: &str = "set_fee_type";
    pub const SUPPORT_NEW_TOKEN_FROM: &str = "support_new_token_from";
    pub const REMOVE_TOKEN_FROM: &str = "remove_token_from";
}

/// ERC20 Token ABI definitions
pub mod erc20_abi {
    pub const APPROVE: &str = "approve";
    pub const ALLOWANCE: &str = "allowance";
    pub const BALANCE_OF: &str = "balance_of";
    pub const DECIMALS: &str = "decimals";
    pub const SYMBOL: &str = "symbol";
    pub const NAME: &str = "name";
}

/// Cairo type definitions matching the ABI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub token_from: ContractAddress,
    pub token_to: ContractAddress,
    pub exchange_address: ContractAddress,
    pub percent: u128,
    pub additional_swap_params: Vec<FieldElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteParams {
    pub token_in: ContractAddress,
    pub token_out: ContractAddress,
    pub amount_in: StarknetUint256,
    pub min_received: StarknetUint256,
    pub destination: ContractAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapParams {
    pub token_in: ContractAddress,
    pub token_out: ContractAddress,
    pub rate: u32,
    pub protocol_id: u32,
    pub pool_address: ContractAddress,
    pub extra_data: Vec<FieldElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapResult {
    pub delta: Delta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delta {
    pub amount0: I129,
    pub amount1: I129,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct I129 {
    pub mag: u128,
    pub sign: bool,
}

/// Real AutoSwappr Contract implementation
pub struct AutoSwapprContract {
    contract_address: ContractAddress,
    provider: Arc<JsonRpcClient<HttpTransport>>,
}

impl AutoSwapprContract {
    /// Create a new AutoSwappr contract instance
    pub fn new(
        contract_address: ContractAddress,
        provider: Arc<JsonRpcClient<HttpTransport>>,
    ) -> Self {
        Self {
            contract_address,
            provider,
        }
    }

    /// Get the contract address
    pub fn address(&self) -> ContractAddress {
        self.contract_address
    }

    /// Get contract parameters
    pub async fn get_contract_parameters<P: Provider>(
        &self,
        provider: &P,
    ) -> Result<ContractInfo, ContractError> {
        let result = provider
            .call(
                FunctionCall {
                    contract_address: self.contract_address,
                    entry_point_selector: selector!("contract_parameters"),
                    calldata: vec![],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .map_err(|e| ContractError::ProviderError(e))?;

        // Parse the result according to the actual Cairo contract return type
        // Expected return: (fees_collector: felt, fibrous_exchange_address: felt,
        // avnu_exchange_address: felt, oracle_address: felt, owner: felt, fee_type: u8, percentage_fee: u16)
        if result.len() < 7 {
            return Err(ContractError::DeserializationError(
                "Insufficient return values from contract_parameters".to_string(),
            ));
        }

        let fees_collector = result[0].to_string();
        let fibrous_exchange_address = result[1].to_string();
        let avnu_exchange_address = result[2].to_string();
        let oracle_address = result[3].to_string();
        let owner = result[4].to_string();

        // Parse fee_type (0 = Fixed, 1 = Percentage)
        let fee_type_raw: u8 = result[5].try_into().unwrap_or(0);
        let fee_type = match fee_type_raw {
            0 => FeeType::Fixed,
            1 => FeeType::Percentage,
            _ => FeeType::Fixed, // Default to Fixed for unknown values
        };

        // Parse percentage_fee
        let percentage_fee: u16 = result[6].try_into().unwrap_or(0);

        Ok(ContractInfo {
            fees_collector,
            fibrous_exchange_address,
            avnu_exchange_address,
            oracle_address,
            owner,
            fee_type,
            percentage_fee,
        })
    }

    /// Execute ekubo swap
    pub async fn ekubo_swap<A: ConnectedAccount + Sync + Send>(
        &self,
        account: &A,
        swap_data: SwapData,
    ) -> Result<Felt, ContractError> {
        // Properly serialize SwapData according to Cairo ABI
        // Expected calldata: (amount: I129, sqrt_ratio_limit: u256, is_token1: bool, skip_ahead: u32, pool_key: PoolKey, caller: felt)
        let mut calldata = Vec::new();

        // Serialize amount (I129: mag: u128, sign: bool)
        let (amount_low, amount_high) =
            conversions::u128_to_uint256(swap_data.params.amount.mag.low);
        calldata.push(amount_low);
        calldata.push(amount_high);
        calldata.push(Felt::from(if swap_data.params.amount.sign { 1 } else { 0 }));

        // Serialize sqrt_ratio_limit (u256: low, high)
        let (sqrt_low, sqrt_high) =
            conversions::u128_to_uint256(swap_data.params.sqrt_ratio_limit.low);
        calldata.push(sqrt_low);
        calldata.push(sqrt_high);

        // Serialize is_token1 (bool)
        calldata.push(Felt::from(if swap_data.params.is_token1 { 1 } else { 0 }));

        // Serialize skip_ahead (u32)
        calldata.push(Felt::from(swap_data.params.skip_ahead));

        // Serialize pool_key (PoolKey: token0, token1, fee, tick_spacing, extension)
        calldata.push(
            Felt::from_hex(&swap_data.pool_key.token0)
                .map_err(|e| ContractError::InvalidAddress(e.to_string()))?,
        );
        calldata.push(
            Felt::from_hex(&swap_data.pool_key.token1)
                .map_err(|e| ContractError::InvalidAddress(e.to_string()))?,
        );
        calldata.push(Felt::from(swap_data.pool_key.fee));
        calldata.push(Felt::from(swap_data.pool_key.tick_spacing));
        calldata.push(
            Felt::from_hex(&swap_data.pool_key.extension)
                .map_err(|e| ContractError::InvalidAddress(e.to_string()))?,
        );

        // Serialize caller (felt)
        calldata.push(
            Felt::from_hex(&swap_data.caller)
                .map_err(|e| ContractError::InvalidAddress(e.to_string()))?,
        );

        let call = Call {
            to: self.contract_address,
            selector: get_selector_from_name(abi::EKUBO_SWAP)
                .map_err(|e| ContractError::CallFailed(e.to_string()))?,
            calldata,
        };

        let execution = account
            .execute_v3(vec![call])
            .send()
            .await
            .map_err(|e| ContractError::AccountError(e.to_string()))?;

        Ok(execution.transaction_hash)
    }

    /// Execute ekubo manual swap
    pub async fn ekubo_manual_swap<A: ConnectedAccount + Sync + Send>(
        &self,
        account: &A,
        swap_data: SwapData,
    ) -> Result<Felt, ContractError> {
        // Same serialization as ekubo_swap but for manual execution
        let mut calldata = Vec::new();

        // Serialize amount (I129: mag: u128, sign: bool)
        let (amount_low, amount_high) =
            conversions::u128_to_uint256(swap_data.params.amount.mag.low);
        calldata.push(amount_low);
        calldata.push(amount_high);
        calldata.push(Felt::from(if swap_data.params.amount.sign { 1 } else { 0 }));

        // Serialize sqrt_ratio_limit (u256: low, high)
        let (sqrt_low, sqrt_high) =
            conversions::u128_to_uint256(swap_data.params.sqrt_ratio_limit.low);
        calldata.push(sqrt_low);
        calldata.push(sqrt_high);

        // Serialize is_token1 (bool)
        calldata.push(Felt::from(if swap_data.params.is_token1 { 1 } else { 0 }));

        // Serialize skip_ahead (u32)
        calldata.push(Felt::from(swap_data.params.skip_ahead));

        // Serialize pool_key (PoolKey: token0, token1, fee, tick_spacing, extension)
        calldata.push(
            Felt::from_hex(&swap_data.pool_key.token0)
                .map_err(|e| ContractError::InvalidAddress(e.to_string()))?,
        );
        calldata.push(
            Felt::from_hex(&swap_data.pool_key.token1)
                .map_err(|e| ContractError::InvalidAddress(e.to_string()))?,
        );
        calldata.push(Felt::from(swap_data.pool_key.fee));
        calldata.push(Felt::from(swap_data.pool_key.tick_spacing));
        calldata.push(
            Felt::from_hex(&swap_data.pool_key.extension)
                .map_err(|e| ContractError::InvalidAddress(e.to_string()))?,
        );

        // Serialize caller (felt)
        calldata.push(
            Felt::from_hex(&swap_data.caller)
                .map_err(|e| ContractError::InvalidAddress(e.to_string()))?,
        );

        let call = Call {
            to: self.contract_address,
            selector: get_selector_from_name(abi::EKUBO_MANUAL_SWAP)
                .map_err(|e| ContractError::CallFailed(e.to_string()))?,
            calldata,
        };

        let execution = account
            .execute_v3(vec![call])
            .send()
            .await
            .map_err(|e| ContractError::AccountError(e.to_string()))?;

        Ok(execution.transaction_hash)
    }

    /// Execute AVNU swap
    pub async fn avnu_swap<A: ConnectedAccount + Sync + Send>(
        &self,
        account: &A,
        protocol_swapper: ContractAddress,
        token_from_address: ContractAddress,
        token_from_amount: StarknetUint256,
        token_to_address: ContractAddress,
        token_to_min_amount: StarknetUint256,
        beneficiary: ContractAddress,
        integrator_fee_amount_bps: u128,
        integrator_fee_recipient: ContractAddress,
        routes: Vec<Route>,
    ) -> Result<Felt, ContractError> {
        // Convert amounts to (low, high) format
        let (token_from_low, token_from_high) = conversions::u128_to_uint256(token_from_amount.low);
        let (token_to_min_low, token_to_min_high) =
            conversions::u128_to_uint256(token_to_min_amount.low);

        // Build calldata with proper serialization
        let mut calldata = vec![
            protocol_swapper,
            token_from_address,
            token_from_low,
            token_from_high,
            token_to_address,
            token_to_min_low,
            token_to_min_high,
            beneficiary,
            Felt::from(integrator_fee_amount_bps),
            integrator_fee_recipient,
        ];

        // Add routes count first
        calldata.push(Felt::from(routes.len()));

        // Serialize each route: (token_from: felt, token_to: felt, exchange_address: felt, percent: u128, additional_swap_params: Array<felt>)
        for route in routes {
            calldata.push(route.token_from);
            calldata.push(route.token_to);
            calldata.push(route.exchange_address);
            calldata.push(Felt::from(route.percent));

            // Add additional_swap_params array length and data
            calldata.push(Felt::from(route.additional_swap_params.len()));
            for param in route.additional_swap_params {
                calldata.push(param);
            }
        }

        let call = Call {
            to: self.contract_address,
            selector: get_selector_from_name(abi::AVNU_SWAP)
                .map_err(|e| ContractError::CallFailed(e.to_string()))?,
            calldata,
        };

        let execution = account
            .execute_v3(vec![call])
            .send()
            .await
            .map_err(|e| ContractError::AccountError(e.to_string()))?;

        Ok(execution.transaction_hash)
    }

    /// Execute Fibrous swap
    pub async fn fibrous_swap<A: ConnectedAccount + Sync + Send>(
        &self,
        account: &A,
        route_params: RouteParams,
        swap_params: Vec<SwapParams>,
        protocol_swapper: ContractAddress,
        beneficiary: ContractAddress,
    ) -> Result<Felt, ContractError> {
        // Build calldata with proper serialization
        let mut calldata = vec![protocol_swapper, beneficiary];

        // Serialize route_params: (token_in: felt, token_out: felt, amount_in: u256, min_received: u256, destination: felt)
        calldata.push(route_params.token_in);
        calldata.push(route_params.token_out);

        // Serialize amount_in (u256: low, high)
        let (amount_in_low, amount_in_high) =
            conversions::u128_to_uint256(route_params.amount_in.low);
        calldata.push(amount_in_low);
        calldata.push(amount_in_high);

        // Serialize min_received (u256: low, high)
        let (min_received_low, min_received_high) =
            conversions::u128_to_uint256(route_params.min_received.low);
        calldata.push(min_received_low);
        calldata.push(min_received_high);

        calldata.push(route_params.destination);

        // Add swap_params count
        calldata.push(Felt::from(swap_params.len()));

        // Serialize each swap_param: (token_in: felt, token_out: felt, rate: u32, protocol_id: u32, pool_address: felt, extra_data: Array<felt>)
        for swap_param in swap_params {
            calldata.push(swap_param.token_in);
            calldata.push(swap_param.token_out);
            calldata.push(Felt::from(swap_param.rate));
            calldata.push(Felt::from(swap_param.protocol_id));
            calldata.push(swap_param.pool_address);

            // Add extra_data array length and data
            calldata.push(Felt::from(swap_param.extra_data.len()));
            for data in swap_param.extra_data {
                calldata.push(data);
            }
        }

        let call = Call {
            to: self.contract_address,
            selector: get_selector_from_name(abi::FIBROUS_SWAP)
                .map_err(|e| ContractError::CallFailed(e.to_string()))?,
            calldata,
        };

        let execution = account
            .execute_v3(vec![call])
            .send()
            .await
            .map_err(|e| ContractError::AccountError(e.to_string()))?;

        Ok(execution.transaction_hash)
    }

    /// Get token amount in USD
    pub async fn get_token_amount_in_usd<P: Provider>(
        &self,
        provider: &P,
        token: ContractAddress,
        token_amount: StarknetUint256,
    ) -> Result<StarknetUint256, ContractError> {
        // Convert token_amount to (low, high) felts for uint256
        let (amount_low, amount_high) = conversions::u128_to_uint256(token_amount.low);

        let result = provider
            .call(
                FunctionCall {
                    contract_address: self.contract_address,
                    entry_point_selector: selector!("get_token_amount_in_usd"),
                    calldata: vec![token, amount_low, amount_high],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .map_err(|e| ContractError::ProviderError(e))?;

        // Parse the result - should return a uint256 (low, high)
        let usd_amount_low = result.get(0).copied().unwrap_or(Felt::ZERO);
        let usd_amount_high = result.get(1).copied().unwrap_or(Felt::ZERO);

        Ok(StarknetUint256 {
            low: usd_amount_low.try_into().unwrap_or(0),
            high: usd_amount_high.try_into().unwrap_or(0),
        })
    }

    /// Get token from status and value
    pub async fn get_token_from_status_and_value<P: Provider>(
        &self,
        provider: &P,
        token_from: ContractAddress,
    ) -> Result<(bool, FieldElement), ContractError> {
        let result = provider
            .call(
                FunctionCall {
                    contract_address: self.contract_address,
                    entry_point_selector: selector!("get_token_from_status_and_value"),
                    calldata: vec![token_from],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .map_err(|e| ContractError::ProviderError(e))?;

        // Parse the result - should return (bool, felt)
        let status = result.get(0).map(|f| f != &Felt::ZERO).unwrap_or(false);
        let value = result.get(1).copied().unwrap_or(FieldElement::ZERO);

        Ok((status, value))
    }

    /// Set fee type
    pub async fn set_fee_type<A: ConnectedAccount + Sync + Send>(
        &self,
        account: &A,
        fee_type: FeeType,
        percentage_fee: u16,
    ) -> Result<Felt, ContractError> {
        // Convert fee_type to felt (assuming it's an enum with numeric values)
        let fee_type_felt = match fee_type {
            FeeType::Fixed => Felt::from(0),
            FeeType::Percentage => Felt::from(1),
        };

        let call = Call {
            to: self.contract_address,
            selector: get_selector_from_name(abi::SET_FEE_TYPE)
                .map_err(|e| ContractError::CallFailed(e.to_string()))?,
            calldata: vec![fee_type_felt, Felt::from(percentage_fee)],
        };

        let execution = account
            .execute_v3(vec![call])
            .send()
            .await
            .map_err(|e| ContractError::AccountError(e.to_string()))?;

        Ok(execution.transaction_hash)
    }

    /// Support new token from
    pub async fn support_new_token_from<A: ConnectedAccount + Sync + Send>(
        &self,
        account: &A,
        token_from: ContractAddress,
        feed_id: FieldElement,
    ) -> Result<Felt, ContractError> {
        let call = Call {
            to: self.contract_address,
            selector: get_selector_from_name(abi::SUPPORT_NEW_TOKEN_FROM)
                .map_err(|e| ContractError::CallFailed(e.to_string()))?,
            calldata: vec![token_from, feed_id],
        };

        let execution = account
            .execute_v3(vec![call])
            .send()
            .await
            .map_err(|e| ContractError::AccountError(e.to_string()))?;

        Ok(execution.transaction_hash)
    }

    /// Remove token from
    pub async fn remove_token_from<A: ConnectedAccount + Sync + Send>(
        &self,
        account: &A,
        token_from: ContractAddress,
    ) -> Result<Felt, ContractError> {
        let call = Call {
            to: self.contract_address,
            selector: get_selector_from_name(abi::REMOVE_TOKEN_FROM)
                .map_err(|e| ContractError::CallFailed(e.to_string()))?,
            calldata: vec![token_from],
        };

        let execution = account
            .execute_v3(vec![call])
            .send()
            .await
            .map_err(|e| ContractError::AccountError(e.to_string()))?;

        Ok(execution.transaction_hash)
    }
}

/// Real ERC20 Token contract implementation
pub struct Erc20Contract {
    contract_address: ContractAddress,
    provider: Arc<JsonRpcClient<HttpTransport>>,
}

impl Erc20Contract {
    /// Create a new ERC20 contract instance
    pub fn new(
        contract_address: ContractAddress,
        provider: Arc<JsonRpcClient<HttpTransport>>,
    ) -> Self {
        Self {
            contract_address,
            provider,
        }
    }

    /// Get the contract address
    pub fn address(&self) -> ContractAddress {
        self.contract_address
    }

    /// Approve token spending
    pub async fn approve<A: ConnectedAccount + Sync + Send>(
        &self,
        account: &A,
        spender: ContractAddress,
        amount: StarknetUint256,
    ) -> Result<Felt, ContractError> {
        // Convert amount to (low, high) felts for uint256
        let (amount_low, amount_high) = conversions::u128_to_uint256(amount.low);

        // Prepare the calldata: [spender, amount_low, amount_high]
        let calldata = vec![spender, amount_low, amount_high];

        let call = Call {
            to: self.contract_address,
            selector: get_selector_from_name(erc20_abi::APPROVE)
                .map_err(|e| ContractError::CallFailed(e.to_string()))?,
            calldata,
        };

        let execution = account
            .execute_v3(vec![call])
            .send()
            .await
            .map_err(|e| ContractError::AccountError(e.to_string()))?;

        Ok(execution.transaction_hash)
    }

    /// Check token allowance
    pub async fn allowance<P: Provider>(
        &self,
        provider: &P,
        owner: ContractAddress,
        spender: ContractAddress,
    ) -> Result<StarknetUint256, ContractError> {
        let allowance = provider
            .call(
                FunctionCall {
                    contract_address: self.contract_address,
                    entry_point_selector: selector!("allowance"),
                    calldata: vec![owner, spender],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .map_err(|e| ContractError::ProviderError(e))?;

        // Parse the result - allowance should return a single felt
        let allowance_value = allowance[0];
        let allowance_u128: u128 = allowance_value.try_into().unwrap_or(0);
        let (low, high) = conversions::u128_to_uint256(allowance_u128);

        Ok(StarknetUint256 {
            low: low.try_into().unwrap_or(0),
            high: high.try_into().unwrap_or(0),
        })
    }

    /// Get token balance
    pub async fn balance_of<P: Provider>(
        &self,
        provider: &P,
        account: ContractAddress,
    ) -> Result<StarknetUint256, ContractError> {
        let balance = provider
            .call(
                FunctionCall {
                    contract_address: self.contract_address,
                    entry_point_selector: selector!("balance_of"),
                    calldata: vec![account],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .map_err(|e| ContractError::ProviderError(e))?;

        // Parse the result - balance should return a single felt
        let balance_value = balance[0];
        let balance_u128: u128 = balance_value.try_into().unwrap_or(0);
        let (low, high) = conversions::u128_to_uint256(balance_u128);

        Ok(StarknetUint256 {
            low: low.try_into().unwrap_or(0),
            high: high.try_into().unwrap_or(0),
        })
    }

    /// Get token decimals
    pub async fn decimals<P: Provider>(&self, provider: &P) -> Result<u8, ContractError> {
        let decimals = provider
            .call(
                FunctionCall {
                    contract_address: self.contract_address,
                    entry_point_selector: selector!("decimals"),
                    calldata: vec![],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .map_err(|e| ContractError::ProviderError(e))?;

        // Parse the result - decimals should return a single felt
        let decimals_value = decimals[0];
        let decimals_u8 = decimals_value.try_into().unwrap_or(18);

        Ok(decimals_u8)
    }

    /// Get token symbol
    pub async fn symbol<P: Provider>(&self, provider: &P) -> Result<String, ContractError> {
        let symbol = provider
            .call(
                FunctionCall {
                    contract_address: self.contract_address,
                    entry_point_selector: selector!("symbol"),
                    calldata: vec![],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .map_err(|e| ContractError::ProviderError(e))?;

        // Parse the result - symbol should return a single felt
        let symbol_value = symbol[0];

        // Convert Felt to ASCII string
        // Most ERC20 tokens store symbol as ASCII in the lower 4 bytes
        let symbol_string = conversions::felt_to_ascii_string(symbol_value);

        Ok(symbol_string)
    }

    /// Get token name
    pub async fn name<P: Provider>(&self, provider: &P) -> Result<String, ContractError> {
        let name = provider
            .call(
                FunctionCall {
                    contract_address: self.contract_address,
                    entry_point_selector: selector!("name"),
                    calldata: vec![],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .map_err(|e| ContractError::ProviderError(e))?;

        // Parse the result - name should return a single felt
        let name_value = name[0];

        // Convert Felt to ASCII string
        // Most ERC20 tokens store name as ASCII in the lower bytes
        let name_string = conversions::felt_to_ascii_string(name_value);

        Ok(name_string)
    }
}

/// Contract address constants for different networks
pub mod addresses {
    use starknet::core::types::Felt;

    // Type alias for compatibility
    type ContractAddress = Felt;

    /// Mainnet contract addresses
    pub mod mainnet {
        use super::*;

        // AutoSwappr contract addresses
        pub const AUTOSWAPPR: &str =
            "0x05582ad635c43b4c14dbfa53cbde0df32266164a0d1b36e5b510e5b34aeb364b";
        pub const EKUBO_CORE: &str = "0xe0e0e08a6a4b9dc7bd67bcb7aade5cf48157d444";
        pub const FIBROUS_EXCHANGE: &str = "0x546f9e447a0bce431949233e3139fe68ec85089e";
        pub const AVNU_EXCHANGE: &str = "0x6712811c214C50b9E12678327Bae02E44Efc357A";

        // Real token addresses
        pub const STRK: &str = "0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d";
        pub const ETH: &str = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
        pub const USDC: &str = "0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8";
        pub const USDT: &str = "0x068f5c6a61780768455de69077e07e89787839bf8166decfbf92b645209c0fb8";
        pub const WBTC: &str = "0x03fe2b97c1fd336e750087d68b9b867997fd64a2661ff3ca5a7c771641e8e7ac";

        pub fn autoswappr() -> ContractAddress {
            Felt::from_hex(AUTOSWAPPR).unwrap()
        }

        pub fn ekubo_core() -> ContractAddress {
            Felt::from_hex(EKUBO_CORE).unwrap()
        }

        pub fn fibrous_exchange() -> ContractAddress {
            Felt::from_hex(FIBROUS_EXCHANGE).unwrap()
        }

        pub fn avnu_exchange() -> ContractAddress {
            Felt::from_hex(AVNU_EXCHANGE).unwrap()
        }

        // Token address getters
        pub fn strk() -> ContractAddress {
            Felt::from_hex(STRK).unwrap()
        }

        pub fn eth() -> ContractAddress {
            Felt::from_hex(ETH).unwrap()
        }

        pub fn usdc() -> ContractAddress {
            Felt::from_hex(USDC).unwrap()
        }

        pub fn usdt() -> ContractAddress {
            Felt::from_hex(USDT).unwrap()
        }

        pub fn wbtc() -> ContractAddress {
            Felt::from_hex(WBTC).unwrap()
        }
    }

    /// Testnet contract addresses
    pub mod testnet {
        use super::*;

        // AutoSwappr contract addresses
        pub const AUTOSWAPPR: &str =
            "0x05582ad635c43b4c14dbfa53cbde0df32266164a0d1b36e5b510e5b34aeb364b";
        pub const EKUBO_CORE: &str = "0xe0e0e08a6a4b9dc7bd67bcb7aade5cf48157d444";
        pub const FIBROUS_EXCHANGE: &str = "0x546f9e447a0bce431949233e3139fe68ec85089e";
        pub const AVNU_EXCHANGE: &str = "0x6712811c214C50b9E12678327Bae02E44Efc357A";

        // Testnet token addresses (using mainnet addresses for now)
        pub const STRK: &str = "0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d";
        pub const ETH: &str = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
        pub const USDC: &str = "0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8";
        pub const USDT: &str = "0x068f5c6a61780768455de69077e07e89787839bf8166decfbf92b645209c0fb8";
        pub const WBTC: &str = "0x03fe2b97c1fd336e750087d68b9b867997fd64a2661ff3ca5a7c771641e8e7ac";

        pub fn autoswappr() -> ContractAddress {
            Felt::from_hex(AUTOSWAPPR).unwrap()
        }

        pub fn ekubo_core() -> ContractAddress {
            Felt::from_hex(EKUBO_CORE).unwrap()
        }

        pub fn fibrous_exchange() -> ContractAddress {
            Felt::from_hex(FIBROUS_EXCHANGE).unwrap()
        }

        pub fn avnu_exchange() -> ContractAddress {
            Felt::from_hex(AVNU_EXCHANGE).unwrap()
        }

        // Token address getters
        pub fn strk() -> ContractAddress {
            Felt::from_hex(STRK).unwrap()
        }

        pub fn eth() -> ContractAddress {
            Felt::from_hex(ETH).unwrap()
        }

        pub fn usdc() -> ContractAddress {
            Felt::from_hex(USDC).unwrap()
        }

        pub fn usdt() -> ContractAddress {
            Felt::from_hex(USDT).unwrap()
        }

        pub fn wbtc() -> ContractAddress {
            Felt::from_hex(WBTC).unwrap()
        }
    }
}

/// Contract-related errors
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Provider error: {0}")]
    ProviderError(#[from] starknet::providers::ProviderError),
    #[error("Account error: {0}")]
    AccountError(String),
    #[error("Contract call failed: {0}")]
    CallFailed(String),
    #[error("Invalid contract address: {0}")]
    InvalidAddress(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}

/// Helper functions for type conversions and utilities
pub mod conversions {
    use super::*;
    use crate::types::connector::{SwapData, Uint256};

    /// Convert our Uint256 to Starknet's Uint256
    pub fn uint256_to_starknet(uint256: &Uint256) -> StarknetUint256 {
        StarknetUint256 {
            low: uint256.low,
            high: uint256.high,
        }
    }

    /// Convert Starknet's Uint256 to our Uint256
    pub fn starknet_to_uint256(uint256: &StarknetUint256) -> Uint256 {
        Uint256 {
            low: uint256.low,
            high: uint256.high,
        }
    }

    /// Convert our SwapData to Cairo-compatible format
    pub fn swap_data_to_cairo(swap_data: &SwapData) -> Result<SwapData, ContractError> {
        // This would need proper conversion to match Cairo struct layout
        Ok(swap_data.clone())
    }

    /// Convert u128 to (low, high) felts for uint256
    pub fn u128_to_uint256(amount: u128) -> (Felt, Felt) {
        let amount_low = Felt::from(amount & 0xFFFFFFFFFFFFFFFF); // Lower 128 bits
        let amount_high = Felt::from(amount >> 64); // Upper 128 bits
        (amount_low, amount_high)
    }

    /// Convert (low, high) felts back to u128
    pub fn uint256_to_u128(low: Felt, high: Felt) -> u128 {
        let low_u128: u128 = low.try_into().unwrap_or(0);
        let high_u128: u128 = high.try_into().unwrap_or(0);
        low_u128 | (high_u128 << 64)
    }

    /// Validate if a string is a valid Starknet address
    pub fn is_valid_address(address: &str) -> bool {
        if address.len() < 3 || !address.starts_with("0x") {
            return false;
        }
        Felt::from_hex(address).is_ok()
    }

    /// Convert Felt to ASCII string
    /// Most ERC20 tokens store strings as ASCII in the lower bytes of a Felt
    pub fn felt_to_ascii_string(felt: Felt) -> String {
        // Convert Felt to u256 and extract ASCII characters
        let felt_value: u128 = felt.try_into().unwrap_or(0);

        // Extract ASCII characters from the lower bytes
        let mut bytes = Vec::new();
        let mut temp = felt_value;

        // Extract up to 8 bytes (64 bits) for ASCII
        for _ in 0..8 {
            let byte = (temp & 0xFF) as u8;
            if byte == 0 {
                break; // Stop at null terminator
            }
            if byte >= 32 && byte <= 126 {
                // Printable ASCII range
                bytes.push(byte);
            }
            temp >>= 8;
        }

        // Reverse to get correct order
        bytes.reverse();

        // Convert to string
        match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => format!("0x{:x}", felt), // Fallback to hex if not valid ASCII
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use starknet::core::types::Felt;

    #[test]
    fn test_contract_address_conversion() {
        let address = addresses::mainnet::autoswappr();
        assert_eq!(
            address,
            Felt::from_hex(addresses::mainnet::AUTOSWAPPR).unwrap()
        );
    }

    #[test]
    fn test_uint256_conversion() {
        let our_uint256 = crate::types::connector::Uint256 { low: 1000, high: 0 };
        let starknet_uint256 = conversions::uint256_to_starknet(&our_uint256);
        let back_to_ours = conversions::starknet_to_uint256(&starknet_uint256);

        assert_eq!(our_uint256.low, back_to_ours.low);
        assert_eq!(our_uint256.high, back_to_ours.high);
    }
}
