use autoswap_rust_sdk::{SimpleAutoSwapprClient, SimpleConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example configuration - replace with your actual values
    let config = SimpleConfig {
        contract_address: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            .to_string(),
        rpc_url: "https://starknet-goerli.public.blastapi.io/rpc/v0_7".to_string(),
        account_address: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            .to_string(),
        private_key: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            .to_string(),
    };

    // Create the client
    let client = SimpleAutoSwapprClient::new(config);

    println!("AutoSwappr client created successfully!");
    println!("Account address: {}", client.account_address());
    println!("Contract address: {}", client.contract_address());

    // Validate configuration
    match client.validate_config() {
        Ok(_) => {
            println!("Configuration is valid");
        }
        Err(e) => {
            println!("Configuration error: {}", e);
            return Err(e.into());
        }
    }

    // Example: Create swap data
    let swap_data = client.create_swap_data(
        "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7", // ETH
        "0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8", // USDC
        "1000000000000000000",                                                // 1 ETH in wei
    )?;

    println!("Created swap data: {:?}", swap_data);

    // Example: Simulate swap
    match client.simulate_swap(&swap_data).await {
        Ok(result) => {
            println!("Swap simulation result: {}", result);
        }
        Err(e) => {
            println!("Failed to simulate swap: {}", e);
        }
    }

    Ok(())
}
