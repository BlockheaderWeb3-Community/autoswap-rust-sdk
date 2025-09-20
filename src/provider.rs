use thiserror::Error;

/// Network configuration for different Starknet networks
#[derive(Debug, Clone)]
pub enum Network {
    Mainnet,
    Testnet,
    Custom(String),
}

impl Network {
    pub fn rpc_url(&self) -> &str {
        match self {
            Network::Mainnet => "https://starknet-mainnet.public.blastapi.io/rpc/v0_7",
            Network::Testnet => "https://starknet-goerli.public.blastapi.io/rpc/v0_7",
            Network::Custom(url) => url,
        }
    }
}

/// Simple provider wrapper for future Starknet integration
#[derive(Debug, Clone)]
pub struct StarknetProvider {
    network: Network,
    rpc_url: String,
}

impl StarknetProvider {
    /// Create a new Starknet provider
    pub fn new(network: Network) -> Result<Self, ProviderError> {
        let rpc_url = network.rpc_url().to_string();

        Ok(Self { network, rpc_url })
    }

    /// Get the network configuration
    pub fn network(&self) -> &Network {
        &self.network
    }

    /// Get the RPC URL
    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }

    /// Validate private key format (basic validation)
    pub fn validate_private_key(&self, private_key: &str) -> Result<(), ProviderError> {
        if private_key.len() < 64 {
            return Err(ProviderError::InvalidPrivateKey(
                "Private key too short".to_string(),
            ));
        }
        if !private_key.starts_with("0x") {
            return Err(ProviderError::InvalidPrivateKey(
                "Private key must start with 0x".to_string(),
            ));
        }
        Ok(())
    }

    /// Validate address format (basic validation)
    pub fn validate_address(&self, address: &str) -> Result<(), ProviderError> {
        if address.len() < 64 {
            return Err(ProviderError::InvalidAddress(
                "Address too short".to_string(),
            ));
        }
        if !address.starts_with("0x") {
            return Err(ProviderError::InvalidAddress(
                "Address must start with 0x".to_string(),
            ));
        }
        Ok(())
    }

    /// Get the chain ID for the current network (placeholder)
    pub async fn chain_id(&self) -> Result<String, ProviderError> {
        match self.network {
            Network::Mainnet => Ok("0x534e5f4d41494e".to_string()), // SN_MAIN
            Network::Testnet => Ok("0x534e5f474f45524c49".to_string()), // SN_GOERLI
            Network::Custom(_) => Ok("0x0".to_string()),
        }
    }

    /// Get the latest block number (placeholder)
    pub async fn block_number(&self) -> Result<u64, ProviderError> {
        // This would make an actual RPC call in a real implementation
        Ok(0)
    }
}

/// Provider-related errors
#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Account creation failed: {0}")]
    AccountCreationFailed(String),
}

// Error conversion will be implemented when AutoSwapprError is available

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_provider_creation() {
        let provider = StarknetProvider::new(Network::Testnet);
        assert!(provider.is_ok());
    }

    #[tokio::test]
    async fn test_chain_id() {
        let provider = StarknetProvider::new(Network::Testnet).unwrap();
        let chain_id = provider.chain_id().await;
        assert!(chain_id.is_ok());
    }
}
