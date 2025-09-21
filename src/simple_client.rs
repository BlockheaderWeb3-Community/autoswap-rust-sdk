use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Simple configuration for the AutoSwappr SDK
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleConfig {
    pub contract_address: String,
    pub rpc_url: String,
    pub account_address: String,
    pub private_key: String,
}

/// Simple error types
#[derive(Error, Debug)]
pub enum SimpleError {
    #[error("Invalid input: {details}")]
    InvalidInput { details: String },
    #[error("Network error: {message}")]
    NetworkError { message: String },
    #[error("Contract error: {message}")]
    ContractError { message: String },
    #[error("{message}")]
    Other { message: String },
}

/// Simple client for AutoSwappr functionality
pub struct SimpleAutoSwapprClient {
    config: SimpleConfig,
}

impl SimpleAutoSwapprClient {
    /// Create a new simple client
    pub fn new(config: SimpleConfig) -> Self {
        Self { config }
    }

    /// Get the contract address
    pub fn contract_address(&self) -> &str {
        &self.config.contract_address
    }

    /// Get the account address
    pub fn account_address(&self) -> &str {
        &self.config.account_address
    }

    /// Get the RPC URL
    pub fn rpc_url(&self) -> &str {
        &self.config.rpc_url
    }

    /// Validate configuration
    pub fn validate_config(&self) -> Result<(), SimpleError> {
        if self.config.contract_address.is_empty() {
            return Err(SimpleError::InvalidInput {
                details: "Contract address cannot be empty".to_string(),
            });
        }

        if self.config.rpc_url.is_empty() {
            return Err(SimpleError::InvalidInput {
                details: "RPC URL cannot be empty".to_string(),
            });
        }

        if self.config.account_address.is_empty() {
            return Err(SimpleError::InvalidInput {
                details: "Account address cannot be empty".to_string(),
            });
        }

        if self.config.private_key.is_empty() {
            return Err(SimpleError::InvalidInput {
                details: "Private key cannot be empty".to_string(),
            });
        }

        Ok(())
    }

    /// Create a basic swap data structure
    pub fn create_swap_data(
        &self,
        token_in: &str,
        token_out: &str,
        amount: &str,
    ) -> Result<SwapData, SimpleError> {
        self.validate_config()?;

        if token_in.is_empty() || token_out.is_empty() || amount.is_empty() {
            return Err(SimpleError::InvalidInput {
                details: "Token addresses and amount cannot be empty".to_string(),
            });
        }

        Ok(SwapData {
            token_in: token_in.to_string(),
            token_out: token_out.to_string(),
            amount: amount.to_string(),
            caller: self.config.account_address.clone(),
        })
    }

    /// Simulate a swap (placeholder for actual implementation)
    pub async fn simulate_swap(&self, swap_data: &SwapData) -> Result<String, SimpleError> {
        self.validate_config()?;

        // This is a placeholder - in a real implementation, this would:
        // 1. Connect to the Starknet network
        // 2. Create the appropriate contract calls
        // 3. Execute the swap transaction
        // 4. Return the transaction hash

        Ok(format!(
            "Simulated swap: {} {} -> {} (amount: {})",
            swap_data.token_in, swap_data.amount, swap_data.token_out, swap_data.amount
        ))
    }
}

/// Simple swap data structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SwapData {
    pub token_in: String,
    pub token_out: String,
    pub amount: String,
    pub caller: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = SimpleConfig {
            contract_address: "0x123".to_string(),
            rpc_url: "https://testnet.starknet.io".to_string(),
            account_address: "0x456".to_string(),
            private_key: "0x789".to_string(),
        };

        let client = SimpleAutoSwapprClient::new(config);
        assert_eq!(client.contract_address(), "0x123");
        assert_eq!(client.account_address(), "0x456");
    }

    #[test]
    fn test_config_validation() {
        let config = SimpleConfig {
            contract_address: "0x123".to_string(),
            rpc_url: "https://testnet.starknet.io".to_string(),
            account_address: "0x456".to_string(),
            private_key: "0x789".to_string(),
        };

        let client = SimpleAutoSwapprClient::new(config);
        assert!(client.validate_config().is_ok());
    }

    #[test]
    fn test_invalid_config() {
        let config = SimpleConfig {
            contract_address: "".to_string(),
            rpc_url: "https://testnet.starknet.io".to_string(),
            account_address: "0x456".to_string(),
            private_key: "0x789".to_string(),
        };

        let client = SimpleAutoSwapprClient::new(config);
        assert!(client.validate_config().is_err());
    }

    #[tokio::test]
    async fn test_swap_data_creation() {
        let config = SimpleConfig {
            contract_address: "0x123".to_string(),
            rpc_url: "https://testnet.starknet.io".to_string(),
            account_address: "0x456".to_string(),
            private_key: "0x789".to_string(),
        };

        let client = SimpleAutoSwapprClient::new(config);
        let swap_data = client
            .create_swap_data("0xabc", "0xdef", "1000000")
            .unwrap();

        assert_eq!(swap_data.token_in, "0xabc");
        assert_eq!(swap_data.token_out, "0xdef");
        assert_eq!(swap_data.amount, "1000000");
    }

    #[tokio::test]
    async fn test_simulate_swap() {
        let config = SimpleConfig {
            contract_address: "0x123".to_string(),
            rpc_url: "https://testnet.starknet.io".to_string(),
            account_address: "0x456".to_string(),
            private_key: "0x789".to_string(),
        };

        let client = SimpleAutoSwapprClient::new(config);
        let swap_data = SwapData {
            token_in: "0xabc".to_string(),
            token_out: "0xdef".to_string(),
            amount: "1000000".to_string(),
            caller: "0x456".to_string(),
        };

        let result = client.simulate_swap(&swap_data).await.unwrap();
        assert!(result.contains("Simulated swap"));
    }
}
