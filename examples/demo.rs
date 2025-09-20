use autoswap_rust_sdk::types::connector::{
    Amount, AutoSwapprConfig, PoolKey, SwapData, SwapParameters, Uint256,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 AutoSwappr Rust SDK - Demo Example");
    println!("=====================================");
    println!("This example demonstrates the SDK without requiring real credentials.");
    println!("It shows the API structure and data types used in the SDK.\n");

    // Create configuration with placeholder values
    let config = AutoSwapprConfig {
        contract_address: "0x05582ad635c43b4c14dbfa53cbde0df32266164a0d1b36e5b510e5b34aeb364b"
            .to_string(),
        rpc_url: "https://starknet-mainnet.public.blastapi.io/rpc/v0_7".to_string(),
        account_address: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            .to_string(),
        private_key: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            .to_string(),
    };

    println!("📋 Configuration created:");
    println!("  Contract Address: {}", config.contract_address);
    println!("  RPC URL: {}", config.rpc_url);
    println!("  Account Address: {}", config.account_address);
    println!("  Private Key: {}...", &config.private_key[..10]);

    // Demonstrate data types
    println!("\n🔧 Data Types Demonstration:");

    // Uint256 example
    let amount = Uint256::from_u128(1000000000000000000u128); // 1 ETH
    println!(
        "  Uint256 from 1 ETH: low={}, high={}",
        amount.low, amount.high
    );

    // Amount example
    let amount_struct = Amount {
        mag: Uint256::from_u128(1000000000000000000u128),
        sign: false,
    };
    println!(
        "  Amount struct: mag.low={}, sign={}",
        amount_struct.mag.low, amount_struct.sign
    );

    // SwapParameters example
    let swap_params = SwapParameters {
        amount: Amount {
            mag: Uint256::from_u128(1000000000000000000u128), // 1 ETH
            sign: false,
        },
        sqrt_ratio_limit: Uint256::from_u128(0),
        is_token1: false,
        skip_ahead: 0,
    };
    println!(
        "  SwapParameters: amount={}, is_token1={}",
        swap_params.amount.mag.low, swap_params.is_token1
    );

    // PoolKey example
    let pool_key = PoolKey {
        token0: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7".to_string(), // ETH
        token1: "0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d".to_string(), // STRK
        fee: 3000,
        tick_spacing: 60,
        extension: "0x0".to_string(),
    };
    println!(
        "  PoolKey: token0={}, token1={}, fee={}",
        pool_key.token0, pool_key.token1, pool_key.fee
    );

    // SwapData example
    let swap_data = SwapData {
        params: swap_params,
        pool_key,
        caller: config.account_address.clone(),
    };
    println!(
        "  SwapData: caller={}, amount={}",
        swap_data.caller, swap_data.params.amount.mag.low
    );

    // Demonstrate address validation
    println!("\n🔍 Address Validation:");
    let valid_addresses = vec![
        "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7", // ETH
        "0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d", // STRK
        "0x05582ad635c43b4c14dbfa53cbde0df32266164a0d1b36e5b510e5b34aeb364b", // AutoSwappr
    ];

    for addr in valid_addresses {
        let is_valid = autoswap_rust_sdk::contracts::conversions::is_valid_address(addr);
        println!(
            "  {}: {}",
            addr,
            if is_valid { "✅ Valid" } else { "❌ Invalid" }
        );
    }

    // Demonstrate token addresses
    println!("\n🪙 Token Addresses:");
    println!(
        "  ETH: {}",
        autoswap_rust_sdk::contracts::addresses::mainnet::ETH
    );
    println!(
        "  STRK: {}",
        autoswap_rust_sdk::contracts::addresses::mainnet::STRK
    );
    println!(
        "  USDC: {}",
        autoswap_rust_sdk::contracts::addresses::mainnet::USDC
    );
    println!(
        "  USDT: {}",
        autoswap_rust_sdk::contracts::addresses::mainnet::USDT
    );
    println!(
        "  WBTC: {}",
        autoswap_rust_sdk::contracts::addresses::mainnet::WBTC
    );

    // Demonstrate contract addresses
    println!("\n📄 Contract Addresses:");
    println!(
        "  AutoSwappr: {}",
        autoswap_rust_sdk::contracts::addresses::mainnet::AUTOSWAPPR
    );
    println!(
        "  Ekubo Core: {}",
        autoswap_rust_sdk::contracts::addresses::mainnet::EKUBO_CORE
    );
    println!(
        "  Fibrous Exchange: {}",
        autoswap_rust_sdk::contracts::addresses::mainnet::FIBROUS_EXCHANGE
    );
    println!(
        "  AVNU Exchange: {}",
        autoswap_rust_sdk::contracts::addresses::mainnet::AVNU_EXCHANGE
    );

    // Demonstrate conversion functions
    println!("\n🔄 Conversion Functions:");
    let test_amount = 1000000000000000000u128; // 1 ETH
    let (low, high) = autoswap_rust_sdk::contracts::conversions::u128_to_uint256(test_amount);
    println!("  u128_to_uint256(1 ETH): low={}, high={}", low, high);

    let back_to_u128 = autoswap_rust_sdk::contracts::conversions::uint256_to_u128(low, high);
    println!("  uint256_to_u128: {}", back_to_u128);

    // Show what the client would do
    println!("\n🚀 Client Capabilities (requires real credentials):");
    println!("  ✅ Create AutoSwapprClient with real Starknet integration");
    println!("  ✅ Get contract parameters from blockchain");
    println!("  ✅ Get token information (name, symbol, decimals)");
    println!("  ✅ Get token balances");
    println!("  ✅ Check token allowances");
    println!("  ✅ Approve tokens for spending");
    println!("  ✅ Execute Ekubo swaps");
    println!("  ✅ Execute AVNU swaps");
    println!("  ✅ Execute Fibrous swaps");
    println!("  ✅ Get USD price conversions");

    println!("\n📝 To use the real client:");
    println!("  1. Set environment variables:");
    println!("     export PRIVATE_KEY=\"your_real_private_key\"");
    println!("     export ACCOUNT_ADDRESS=\"your_real_account_address\"");
    println!("  2. Run: cargo run --example advanced_usage");

    println!("\n🎉 Demo completed successfully!");
    println!("   The SDK is ready for real Starknet integration!");

    Ok(())
}
