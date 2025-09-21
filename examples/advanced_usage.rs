use autoswap_rust_sdk::{
    client::AutoSwapprClient,
    types::connector::{Amount, AutoSwapprConfig, PoolKey, SwapData, SwapParameters, Uint256},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AutoSwappr Rust SDK - Advanced Client Usage Example");
    println!("=====================================================");

    // Get configuration from environment variables
    let rpc_url = env::var("RPC_URL")
        .unwrap_or_else(|_| "https://starknet-mainnet.public.blastapi.io/rpc/v0_7".to_string());

    // For demo purposes, use placeholder values if environment variables are not set
    let private_key = env::var("PRIVATE_KEY").unwrap_or_else(|_| {
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string()
    });
    let account_address = env::var("ACCOUNT_ADDRESS").unwrap_or_else(|_| {
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string()
    });
    let contract_address = env::var("CONTRACT_ADDRESS").unwrap_or_else(|_| {
        "0x05582ad635c43b4c14dbfa53cbde0df32266164a0d1b36e5b510e5b34aeb364b".to_string()
    });

    // Check if we're using placeholder values
    let using_placeholders =
        private_key == "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    println!("RPC URL: {}", rpc_url);
    println!("Account: {}", account_address);
    println!("Contract: {}", contract_address);

    if using_placeholders {
        println!("\nWARNING: Using placeholder values for demonstration!");
        println!("   To use real values, set these environment variables:");
        println!("   export PRIVATE_KEY=\"your_real_private_key\"");
        println!("   export ACCOUNT_ADDRESS=\"your_real_account_address\"");
        println!(
            "   export CONTRACT_ADDRESS=\"0x05582ad635c43b4c14dbfa53cbde0df32266164a0d1b36e5b510e5b34aeb364b\""
        );
        println!("\n   This example will demonstrate the API but may fail on real network calls.");
    }

    // Create configuration
    let config = AutoSwapprConfig {
        contract_address: contract_address.clone(),
        rpc_url,
        account_address: account_address.clone(),
        private_key,
    };

    // Create client with real Starknet integration
    let client = match AutoSwapprClient::new(config).await {
        Ok(client) => {
            println!("\nClient created successfully!");
            client
        }
        Err(e) => {
            if using_placeholders {
                println!(
                    "\nClient creation failed (expected with placeholder values): {}",
                    e
                );
                println!("\nThis demonstrates that the client requires valid credentials.");
                println!("   The API is working correctly - you just need real values!");
                return Ok(());
            } else {
                return Err(e.into());
            }
        }
    };

    println!("Account Address: {}", client.account_address());
    println!("Contract Address: {}", client.contract_address());

    // Example 1: Get contract parameters
    println!("\nGetting AutoSwappr contract parameters...");
    match client.get_contract_parameters().await {
        Ok(params) => {
            println!("Contract parameters retrieved:");
            println!("  Fees Collector: {}", params.fees_collector);
            println!("  Fibrous Exchange: {}", params.fibrous_exchange_address);
            println!("  AVNU Exchange: {}", params.avnu_exchange_address);
            println!("  Oracle: {}", params.oracle_address);
            println!("  Owner: {}", params.owner);
            println!("  Fee Type: {:?}", params.fee_type);
            println!("  Percentage Fee: {}", params.percentage_fee);
        }
        Err(e) => println!("Failed to get contract parameters: {}", e),
    }

    // Example 2: Get token information
    println!("\n Getting token information...");

    let eth_address = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
    let strk_address = "0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d";

    // Get ETH token info
    match client.get_token_info(eth_address).await {
        Ok((name, symbol, decimals)) => {
            println!(" ETH Token Info:");
            println!("  Name: {}", name);
            println!("  Symbol: {}", symbol);
            println!("  Decimals: {}", decimals);
        }
        Err(e) => println!(" Failed to get ETH token info: {}", e),
    }

    // Get STRK token info
    match client.get_token_info(strk_address).await {
        Ok((name, symbol, decimals)) => {
            println!(" STRK Token Info:");
            println!("  Name: {}", name);
            println!("  Symbol: {}", symbol);
            println!("  Decimals: {}", decimals);
        }
        Err(e) => println!(" Failed to get STRK token info: {}", e),
    }

    // Example 3: Get token balances
    println!("\n Getting token balances...");

    match client.get_token_balance(eth_address).await {
        Ok(balance) => {
            println!(" ETH Balance: {}", balance);
        }
        Err(e) => println!(" Failed to get ETH balance: {}", e),
    }

    match client.get_token_balance(strk_address).await {
        Ok(balance) => {
            println!(" STRK Balance: {}", balance);
        }
        Err(e) => println!(" Failed to get STRK balance: {}", e),
    }

    // Example 4: Get token allowances
    println!("\n Getting token allowances...");

    match client
        .get_allowance(eth_address, &account_address, &contract_address)
        .await
    {
        Ok(allowance) => {
            println!(" ETH Allowance for AutoSwappr: {}", allowance);
        }
        Err(e) => println!(" Failed to get ETH allowance: {}", e),
    }

    // Example 5: Get token amount in USD
    println!("\n Getting token amounts in USD...");

    let test_amount = 1000000000000000000u128; // 1 ETH (18 decimals)
    match client
        .get_token_amount_in_usd_formatted(eth_address, test_amount, 18)
        .await
    {
        Ok(usd_amount) => {
            println!(" 1 ETH in USD: ${:.2}", usd_amount);
        }
        Err(e) => println!(" Failed to get ETH USD amount: {}", e),
    }

    // Also show the raw value for comparison
    match client
        .get_token_amount_in_usd(eth_address, test_amount)
        .await
    {
        Ok(raw_usd_amount) => {
            println!(
                "   Raw value: {} (shows why formatting is needed)",
                raw_usd_amount
            );
        }
        Err(e) => println!(" Failed to get raw ETH USD amount: {}", e),
    }

    // Example 6: Create swap data for demonstration
    println!("\n Creating swap data for demonstration...");

    let swap_data = SwapData {
        params: SwapParameters {
            amount: Amount {
                mag: Uint256::from_u128(1000000000000000000), // 1 ETH
                sign: false,
            },
            sqrt_ratio_limit: Uint256::from_u128(0),
            is_token1: false,
            skip_ahead: 0,
        },
        pool_key: PoolKey {
            token0: eth_address.to_string(),
            token1: strk_address.to_string(),
            fee: 3000,
            tick_spacing: 60,
            extension: "0x0".to_string(),
        },
        caller: client.account_address(),
    };

    println!(" Swap data created:");
    println!("  Amount: {} ETH", swap_data.params.amount.mag.low);
    println!("  Token0: {}", swap_data.pool_key.token0);
    println!("  Token1: {}", swap_data.pool_key.token1);
    println!("  Fee: {} bps", swap_data.pool_key.fee);

    // Example 7: Demonstrate token approval (commented out for safety)
    println!("\n  Token approval example (commented out for safety):");
    println!("   To approve tokens for swapping, you would call:");
    println!("   client.approve_token(eth_address, &client.contract_address(), amount).await");
    println!("   This would return a transaction hash upon success.");

    // Example 8: Demonstrate swap execution (commented out for safety)
    println!("\n  Swap execution example (commented out for safety):");
    println!("   To execute a swap, you would call:");
    println!("   client.execute_ekubo_swap(swap_data).await");
    println!("   This would return a transaction hash upon success.");

    // Example 9: Demonstrate advanced features
    println!("\n Advanced features available:");
    println!("  - execute_avnu_swap() - Execute AVNU protocol swaps");
    println!("  - execute_fibrous_swap() - Execute Fibrous protocol swaps");
    println!("  - execute_swap_with_approval() - Complete swap with automatic approval");
    println!("  - get_token_info() - Get comprehensive token information");
    println!("  - get_token_balance() - Get token balances");
    println!("  - get_allowance() - Check token allowances");

    // Example 10: Show how to access underlying components
    println!("\n Accessing underlying components:");
    println!("  - client.account() - Access the Starknet account");
    println!("  - client.autoswappr_contract() - Access the AutoSwappr contract");
    println!("  - client.provider() - Access the Starknet provider");

    println!("\n Advanced example completed successfully!");
    println!("\n Note: This example demonstrates read-only operations.");
    println!("   For actual swaps, you would need to:");
    println!("   1. Approve tokens for the AutoSwappr contract");
    println!("   2. Execute the swap transaction");
    println!("   3. Monitor the transaction status");
    println!("\n The client provides both high-level convenience methods");
    println!("   and low-level access to underlying Starknet components.");

    Ok(())
}
