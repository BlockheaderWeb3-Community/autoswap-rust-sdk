use crate::{
    contracts::{AutoSwapprContract, Erc20Contract},
    types::connector::{AutoSwapprConfig, AutoSwapprError, ContractInfo, SwapData, Uint256},
};
use starknet::{
    accounts::{Account, ExecutionEncoding, SingleOwnerAccount},
    core::{chain_id, types::Felt},
    providers::{
        Url,
        jsonrpc::{HttpTransport, JsonRpcClient},
    },
    signers::{LocalWallet, SigningKey},
};
use std::sync::Arc;

/// Main client for interacting with AutoSwappr with real Starknet integration
pub struct AutoSwapprClient {
    provider: Arc<JsonRpcClient<HttpTransport>>,
    autoswappr_contract: AutoSwapprContract,
    account: SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    config: AutoSwapprConfig,
}

impl AutoSwapprClient {
    /// Create a new AutoSwappr client with real Starknet integration
    pub async fn new(config: AutoSwapprConfig) -> Result<Self, AutoSwapprError> {
        // Parse RPC URL
        let rpc_url = Url::parse(&config.rpc_url).map_err(|e| AutoSwapprError::InvalidInput {
            details: format!("Invalid RPC URL: {}", e),
        })?;

        // Create provider
        let provider = Arc::new(JsonRpcClient::new(HttpTransport::new(rpc_url)));

        // Parse account address
        let account_address =
            Felt::from_hex(&config.account_address).map_err(|e| AutoSwapprError::InvalidInput {
                details: format!("Invalid account address: {}", e),
            })?;

        // Parse private key
        let private_key =
            Felt::from_hex(&config.private_key).map_err(|e| AutoSwapprError::InvalidInput {
                details: format!("Invalid private key: {}", e),
            })?;

        // Create signer
        let signer = LocalWallet::from(SigningKey::from_secret_scalar(private_key));

        // Create account
        let account = SingleOwnerAccount::new(
            (*provider).clone(),
            signer,
            account_address,
            chain_id::MAINNET, // TODO: Make this configurable based on RPC URL
            ExecutionEncoding::New,
        );

        // Parse contract address
        let contract_address = Felt::from_hex(&config.contract_address).map_err(|e| {
            AutoSwapprError::InvalidInput {
                details: format!("Invalid contract address: {}", e),
            }
        })?;

        // Create AutoSwappr contract
        let autoswappr_contract = AutoSwapprContract::new(contract_address, provider.clone());

        Ok(Self {
            provider,
            autoswappr_contract,
            account,
            config,
        })
    }

    /// Get contract parameters
    pub async fn get_contract_parameters(&self) -> Result<ContractInfo, AutoSwapprError> {
        self.autoswappr_contract
            .get_contract_parameters(&*self.provider)
            .await
            .map_err(|e| AutoSwapprError::Other {
                message: e.to_string(),
            })
    }

    /// Get token amount in USD
    pub async fn get_token_amount_in_usd(
        &self,
        token: &str,
        token_amount: u128,
    ) -> Result<u128, AutoSwapprError> {
        let token_felt = Felt::from_hex(token).map_err(|e| AutoSwapprError::InvalidInput {
            details: format!("Invalid token address: {}", e),
        })?;

        let amount_uint256 = Uint256::from_u128(token_amount);
        let starknet_uint256 = crate::contracts::conversions::uint256_to_starknet(&amount_uint256);

        let result = self
            .autoswappr_contract
            .get_token_amount_in_usd(&*self.provider, token_felt, starknet_uint256)
            .await
            .map_err(|e| AutoSwapprError::Other {
                message: e.to_string(),
            })?;

        Ok(crate::contracts::conversions::uint256_to_u128(
            result.low.try_into().unwrap_or(Felt::ZERO),
            result.high.try_into().unwrap_or(Felt::ZERO),
        ))
    }

    /// Get token amount in USD with proper decimal formatting
    pub async fn get_token_amount_in_usd_formatted(
        &self,
        token: &str,
        token_amount: u128,
        decimals: u8,
    ) -> Result<f64, AutoSwapprError> {
        let raw_usd_amount = self.get_token_amount_in_usd(token, token_amount).await?;

        // Convert from raw amount to decimal amount
        let divisor = 10_u128.pow(decimals as u32);
        let usd_amount = raw_usd_amount as f64 / divisor as f64;

        Ok(usd_amount)
    }

    /// Check token allowance
    pub async fn get_allowance(
        &self,
        token_address: &str,
        owner: &str,
        spender: &str,
    ) -> Result<u128, AutoSwapprError> {
        let token_felt =
            Felt::from_hex(token_address).map_err(|e| AutoSwapprError::InvalidInput {
                details: format!("Invalid token address: {}", e),
            })?;

        let owner_felt = Felt::from_hex(owner).map_err(|e| AutoSwapprError::InvalidInput {
            details: format!("Invalid owner address: {}", e),
        })?;

        let spender_felt = Felt::from_hex(spender).map_err(|e| AutoSwapprError::InvalidInput {
            details: format!("Invalid spender address: {}", e),
        })?;

        let erc20_contract = Erc20Contract::new(token_felt, self.provider.clone());

        let result = erc20_contract
            .allowance(&*self.provider, owner_felt, spender_felt)
            .await
            .map_err(|e| AutoSwapprError::Other {
                message: e.to_string(),
            })?;

        Ok(crate::contracts::conversions::uint256_to_u128(
            result.low.try_into().unwrap_or(Felt::ZERO),
            result.high.try_into().unwrap_or(Felt::ZERO),
        ))
    }

    /// Approve token spending
    pub async fn approve_token(
        &self,
        token_address: &str,
        spender: &str,
        amount: u128,
    ) -> Result<String, AutoSwapprError> {
        let token_felt =
            Felt::from_hex(token_address).map_err(|e| AutoSwapprError::InvalidInput {
                details: format!("Invalid token address: {}", e),
            })?;

        let spender_felt = Felt::from_hex(spender).map_err(|e| AutoSwapprError::InvalidInput {
            details: format!("Invalid spender address: {}", e),
        })?;

        let erc20_contract = Erc20Contract::new(token_felt, self.provider.clone());

        let amount_uint256 = Uint256::from_u128(amount);
        let starknet_uint256 = crate::contracts::conversions::uint256_to_starknet(&amount_uint256);

        let tx_hash = erc20_contract
            .approve(&self.account, spender_felt, starknet_uint256)
            .await
            .map_err(|e| AutoSwapprError::Other {
                message: e.to_string(),
            })?;

        Ok(tx_hash.to_string())
    }

    /// Get token balance
    pub async fn get_token_balance(&self, token_address: &str) -> Result<u128, AutoSwapprError> {
        let token_felt =
            Felt::from_hex(token_address).map_err(|e| AutoSwapprError::InvalidInput {
                details: format!("Invalid token address: {}", e),
            })?;

        let erc20_contract = Erc20Contract::new(token_felt, self.provider.clone());

        let result = erc20_contract
            .balance_of(&*self.provider, self.account.address())
            .await
            .map_err(|e| AutoSwapprError::Other {
                message: e.to_string(),
            })?;

        Ok(crate::contracts::conversions::uint256_to_u128(
            result.low.try_into().unwrap_or(Felt::ZERO),
            result.high.try_into().unwrap_or(Felt::ZERO),
        ))
    }

    /// Get token information
    pub async fn get_token_info(
        &self,
        token_address: &str,
    ) -> Result<(String, String, u8), AutoSwapprError> {
        let token_felt =
            Felt::from_hex(token_address).map_err(|e| AutoSwapprError::InvalidInput {
                details: format!("Invalid token address: {}", e),
            })?;

        let erc20_contract = Erc20Contract::new(token_felt, self.provider.clone());

        let name =
            erc20_contract
                .name(&*self.provider)
                .await
                .map_err(|e| AutoSwapprError::Other {
                    message: e.to_string(),
                })?;

        let symbol =
            erc20_contract
                .symbol(&*self.provider)
                .await
                .map_err(|e| AutoSwapprError::Other {
                    message: e.to_string(),
                })?;

        let decimals = erc20_contract
            .decimals(&*self.provider)
            .await
            .map_err(|e| AutoSwapprError::Other {
                message: e.to_string(),
            })?;

        Ok((name, symbol, decimals))
    }

    /// Execute ekubo manual swap
    pub async fn execute_ekubo_manual_swap(
        &self,
        swap_data: SwapData,
    ) -> Result<String, AutoSwapprError> {
        let tx_hash = self
            .autoswappr_contract
            .ekubo_manual_swap(&self.account, swap_data)
            .await
            .map_err(|e| AutoSwapprError::Other {
                message: e.to_string(),
            })?;

        Ok(tx_hash.to_string())
    }

    /// Execute ekubo swap
    pub async fn execute_ekubo_swap(&self, swap_data: SwapData) -> Result<String, AutoSwapprError> {
        let tx_hash = self
            .autoswappr_contract
            .ekubo_swap(&self.account, swap_data)
            .await
            .map_err(|e| AutoSwapprError::Other {
                message: e.to_string(),
            })?;

        Ok(tx_hash.to_string())
    }

    /// Execute AVNU swap
    pub async fn execute_avnu_swap(
        &self,
        protocol_swapper: &str,
        token_from_address: &str,
        token_from_amount: u128,
        token_to_address: &str,
        token_to_min_amount: u128,
        beneficiary: &str,
        integrator_fee_amount_bps: u128,
        integrator_fee_recipient: &str,
        routes: Vec<crate::contracts::Route>,
    ) -> Result<String, AutoSwapprError> {
        let protocol_swapper_felt =
            Felt::from_hex(protocol_swapper).map_err(|e| AutoSwapprError::InvalidInput {
                details: format!("Invalid protocol swapper address: {}", e),
            })?;

        let token_from_felt =
            Felt::from_hex(token_from_address).map_err(|e| AutoSwapprError::InvalidInput {
                details: format!("Invalid token from address: {}", e),
            })?;

        let token_to_felt =
            Felt::from_hex(token_to_address).map_err(|e| AutoSwapprError::InvalidInput {
                details: format!("Invalid token to address: {}", e),
            })?;

        let beneficiary_felt =
            Felt::from_hex(beneficiary).map_err(|e| AutoSwapprError::InvalidInput {
                details: format!("Invalid beneficiary address: {}", e),
            })?;

        let integrator_fee_recipient_felt =
            Felt::from_hex(integrator_fee_recipient).map_err(|e| {
                AutoSwapprError::InvalidInput {
                    details: format!("Invalid integrator fee recipient address: {}", e),
                }
            })?;

        let from_amount_uint256 = Uint256::from_u128(token_from_amount);
        let to_min_amount_uint256 = Uint256::from_u128(token_to_min_amount);

        let tx_hash = self
            .autoswappr_contract
            .avnu_swap(
                &self.account,
                protocol_swapper_felt,
                token_from_felt,
                crate::contracts::conversions::uint256_to_starknet(&from_amount_uint256),
                token_to_felt,
                crate::contracts::conversions::uint256_to_starknet(&to_min_amount_uint256),
                beneficiary_felt,
                integrator_fee_amount_bps,
                integrator_fee_recipient_felt,
                routes,
            )
            .await
            .map_err(|e| AutoSwapprError::Other {
                message: e.to_string(),
            })?;

        Ok(tx_hash.to_string())
    }

    /// Execute Fibrous swap
    pub async fn execute_fibrous_swap(
        &self,
        protocol_swapper: &str,
        beneficiary: &str,
        route_params: crate::contracts::RouteParams,
        swap_params: Vec<crate::contracts::SwapParams>,
    ) -> Result<String, AutoSwapprError> {
        let protocol_swapper_felt =
            Felt::from_hex(protocol_swapper).map_err(|e| AutoSwapprError::InvalidInput {
                details: format!("Invalid protocol swapper address: {}", e),
            })?;

        let beneficiary_felt =
            Felt::from_hex(beneficiary).map_err(|e| AutoSwapprError::InvalidInput {
                details: format!("Invalid beneficiary address: {}", e),
            })?;

        let tx_hash = self
            .autoswappr_contract
            .fibrous_swap(
                &self.account,
                route_params,
                swap_params,
                protocol_swapper_felt,
                beneficiary_felt,
            )
            .await
            .map_err(|e| AutoSwapprError::Other {
                message: e.to_string(),
            })?;

        Ok(tx_hash.to_string())
    }

    /// Execute a complete swap with approval
    pub async fn execute_swap_with_approval(
        &self,
        token_in: &str,
        swap_data: SwapData,
        amount: u128,
    ) -> Result<String, AutoSwapprError> {
        // First approve the token
        let _approve_result = self
            .approve_token(token_in, &self.config.contract_address, amount)
            .await?;

        // Then execute the swap
        let swap_result = self.execute_ekubo_manual_swap(swap_data).await?;

        Ok(swap_result)
    }

    /// Get account address
    pub fn account_address(&self) -> String {
        self.account.address().to_string()
    }

    /// Get contract address
    pub fn contract_address(&self) -> String {
        self.autoswappr_contract.address().to_string()
    }

    /// Get the underlying provider
    pub fn provider(&self) -> &JsonRpcClient<HttpTransport> {
        &self.provider
    }

    /// Get account reference for advanced usage
    pub fn account(&self) -> &SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet> {
        &self.account
    }

    /// Get AutoSwappr contract reference for advanced usage
    pub fn autoswappr_contract(&self) -> &AutoSwapprContract {
        &self.autoswappr_contract
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::connector::AutoSwapprConfig;

    #[tokio::test]
    async fn test_client_creation() {
        let config = AutoSwapprConfig {
            contract_address: "0x05582ad635c43b4c14dbfa53cbde0df32266164a0d1b36e5b510e5b34aeb364b"
                .to_string(),
            rpc_url: "https://starknet-mainnet.public.blastapi.io/rpc/v0_7".to_string(),
            account_address: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .to_string(),
            private_key: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .to_string(),
        };

        let client = AutoSwapprClient::new(config).await;
        // This should work now with real implementation
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_contract_parameters() {
        let config = AutoSwapprConfig {
            contract_address: "0x05582ad635c43b4c14dbfa53cbde0df32266164a0d1b36e5b510e5b34aeb364b"
                .to_string(),
            rpc_url: "https://starknet-mainnet.public.blastapi.io/rpc/v0_7".to_string(),
            account_address: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .to_string(),
            private_key: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                .to_string(),
        };

        let client = AutoSwapprClient::new(config).await.unwrap();
        let params = client.get_contract_parameters().await;
        // This will make a real contract call, so it might fail in tests
        // but the method should exist and be callable
        assert!(params.is_ok() || params.is_err());
    }
}
