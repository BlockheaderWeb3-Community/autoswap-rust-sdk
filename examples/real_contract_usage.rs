use autoswap_rust_sdk::{
    contracts::{AutoSwapprContract, Erc20Contract, addresses},
    types::connector::{Amount, PoolKey, SwapData, SwapParameters, Uint256},
};
use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    core::{chain_id, types::Felt},
    providers::{
        Url,
        jsonrpc::{HttpTransport, JsonRpcClient},
    },
    signers::{LocalWallet, SigningKey},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" AutoSwappr Rust SDK - Real Contract Usage Example");
    println!("=====================================================");

    // Get configuration from environment variables
    let rpc_url = env::var("RPC_URL")
        .unwrap_or_else(|_| "https://starknet-mainnet.public.blastapi.io/rpc/v0_7".to_string());
    let private_key =
        env::var("PRIVATE_KEY").expect("PRIVATE_KEY environment variable is required");
    let account_address =
        env::var("ACCOUNT_ADDRESS").expect("ACCOUNT_ADDRESS environment variable is required");

    println!(" RPC URL: {}", rpc_url);
    println!(" Account: {}", account_address);

    // Create provider
    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url)?));

    // Create account
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(Felt::from_hex(
        &private_key,
    )?));
    let address = Felt::from_hex(&account_address)?;
    let _account = SingleOwnerAccount::new(
        provider.clone(),
        signer,
        address,
        chain_id::MAINNET,
        ExecutionEncoding::New,
    );

    // Create contract instances
    let autoswappr_contract = AutoSwapprContract::new(
        addresses::mainnet::autoswappr(),
        std::sync::Arc::new(provider.clone()),
    );

    let eth_contract = Erc20Contract::new(
        addresses::mainnet::eth(),
        std::sync::Arc::new(provider.clone()),
    );

    let strk_contract = Erc20Contract::new(
        addresses::mainnet::strk(),
        std::sync::Arc::new(provider.clone()),
    );

    println!("\n📋 Contract Addresses:");
    println!("AutoSwappr: {}", autoswappr_contract.address());
    println!("ETH: {}", eth_contract.address());
    println!("STRK: {}", strk_contract.address());

    // Example 1: Get contract parameters
    println!("\n Getting AutoSwappr contract parameters...");
    match autoswappr_contract.get_contract_parameters(&provider).await {
        Ok(params) => {
            println!(" Contract parameters retrieved:");
            println!("  Fees Collector: {}", params.fees_collector);
            println!("  Fibrous Exchange: {}", params.fibrous_exchange_address);
            println!("  AVNU Exchange: {}", params.avnu_exchange_address);
            println!("  Oracle: {}", params.oracle_address);
            println!("  Owner: {}", params.owner);
            println!("  Fee Type: {:?}", params.fee_type);
            println!("  Percentage Fee: {}", params.percentage_fee);
        }
        Err(e) => println!(" Failed to get contract parameters: {}", e),
    }

    // Example 2: Get token information
    println!("\n Getting token information...");

    // Get ETH token info
    match eth_contract.name(&provider).await {
        Ok(name) => println!(" ETH Name: {}", name),
        Err(e) => println!(" Failed to get ETH name: {}", e),
    }

    match eth_contract.symbol(&provider).await {
        Ok(symbol) => println!(" ETH Symbol: {}", symbol),
        Err(e) => println!(" Failed to get ETH symbol: {}", e),
    }

    match eth_contract.decimals(&provider).await {
        Ok(decimals) => println!(" ETH Decimals: {}", decimals),
        Err(e) => println!(" Failed to get ETH decimals: {}", e),
    }

    // Get STRK token info
    match strk_contract.name(&provider).await {
        Ok(name) => println!(" STRK Name: {}", name),
        Err(e) => println!(" Failed to get STRK name: {}", e),
    }

    match strk_contract.symbol(&provider).await {
        Ok(symbol) => println!(" STRK Symbol: {}", symbol),
        Err(e) => println!(" Failed to get STRK symbol: {}", e),
    }

    // Example 3: Get token balances
    println!("\n Getting token balances...");

    match eth_contract.balance_of(&provider, address).await {
        Ok(balance) => {
            let balance_u128 = autoswap_rust_sdk::contracts::conversions::uint256_to_u128(
                balance.low.try_into().unwrap_or(Felt::ZERO),
                balance.high.try_into().unwrap_or(Felt::ZERO),
            );
            println!(" ETH Balance: {}", balance_u128);
        }
        Err(e) => println!(" Failed to get ETH balance: {}", e),
    }

    match strk_contract.balance_of(&provider, address).await {
        Ok(balance) => {
            let balance_u128 = autoswap_rust_sdk::contracts::conversions::uint256_to_u128(
                balance.low.try_into().unwrap_or(Felt::ZERO),
                balance.high.try_into().unwrap_or(Felt::ZERO),
            );
            println!(" STRK Balance: {}", balance_u128);
        }
        Err(e) => println!(" Failed to get STRK balance: {}", e),
    }

    // Example 4: Get token allowance
    println!("\n Getting token allowances...");

    match eth_contract
        .allowance(&provider, address, autoswappr_contract.address())
        .await
    {
        Ok(allowance) => {
            let allowance_u128 = autoswap_rust_sdk::contracts::conversions::uint256_to_u128(
                allowance.low.try_into().unwrap_or(Felt::ZERO),
                allowance.high.try_into().unwrap_or(Felt::ZERO),
            );
            println!(" ETH Allowance for AutoSwappr: {}", allowance_u128);
        }
        Err(e) => println!(" Failed to get ETH allowance: {}", e),
    }

    // Example 5: Get token amount in USD
    println!("\n Getting token amounts in USD...");

    let test_amount = Uint256::from_u128(1000000000000000000); // 1 ETH (18 decimals)
    match autoswappr_contract
        .get_token_amount_in_usd(&provider, addresses::mainnet::eth(), test_amount)
        .await
    {
        Ok(usd_amount) => {
            let usd_u128 = autoswap_rust_sdk::contracts::conversions::uint256_to_u128(
                usd_amount.low.try_into().unwrap_or(Felt::ZERO),
                usd_amount.high.try_into().unwrap_or(Felt::ZERO),
            );
            println!(" 1 ETH in USD: ${}", usd_u128);
        }
        Err(e) => println!(" Failed to get ETH USD amount: {}", e),
    }

    // Example 6: Create swap data (for demonstration)
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
            token0: addresses::mainnet::eth().to_string(),
            token1: addresses::mainnet::strk().to_string(),
            fee: 3000,
            tick_spacing: 60,
            extension: "0x0".to_string(),
        },
        caller: account_address.clone(),
    };

    println!(" Swap data created:");
    println!("  Amount: {} ETH", swap_data.params.amount.mag.low);
    println!("  Token0: {}", swap_data.pool_key.token0);
    println!("  Token1: {}", swap_data.pool_key.token1);
    println!("  Fee: {} bps", swap_data.pool_key.fee);

    // Example 7: Demonstrate token approval (commented out for safety)
    println!("\n  Token approval example (commented out for safety):");
    println!("   To approve tokens for swapping, you would call:");
    println!("   eth_contract.approve(&account, autoswappr_contract.address(), amount).await");
    println!("   This would return a transaction hash upon success.");

    // Example 8: Demonstrate swap execution (commented out for safety)
    println!("\n  Swap execution example (commented out for safety):");
    println!("   To execute a swap, you would call:");
    println!("   autoswappr_contract.ekubo_swap(&account, swap_data).await");
    println!("   This would return a transaction hash upon success.");

    println!("\n Example completed successfully!");
    println!("\n Note: This example demonstrates read-only operations.");
    println!("   For actual swaps, you would need to:");
    println!("   1. Approve tokens for the AutoSwappr contract");
    println!("   2. Execute the swap transaction");
    println!("   3. Monitor the transaction status");

    Ok(())
}
