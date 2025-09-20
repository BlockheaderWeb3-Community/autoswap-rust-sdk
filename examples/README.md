# AutoSwappr Rust SDK Examples

This directory contains examples demonstrating how to use the AutoSwappr Rust SDK.

## Examples

### 1. `real_contract_usage.rs` - Real Contract Interactions

This example demonstrates how to interact with real Starknet contracts using the AutoSwappr SDK.

**Features demonstrated:**
- ✅ Reading contract parameters
- ✅ Getting token information (name, symbol, decimals)
- ✅ Checking token balances
- ✅ Checking token allowances
- ✅ Getting token amounts in USD
- ✅ Creating swap data structures
- ⚠️ Token approval (commented out for safety)
- ⚠️ Swap execution (commented out for safety)

**Prerequisites:**
- Set environment variables:
  ```bash
  export RPC_URL="https://starknet-mainnet.public.blastapi.io/rpc/v0_7"
  export PRIVATE_KEY="your_private_key_here"
  export ACCOUNT_ADDRESS="your_account_address_here"
  ```

**Run the example:**
```bash
cargo run --example real_contract_usage
```

### 2. `basic_usage.rs` - Simple Client Usage

This example demonstrates basic usage of the simplified client for testing and development.

**Run the example:**
```bash
cargo run --example basic_usage
```

### 3. `advanced_usage.rs` - Advanced Client Usage

This example demonstrates advanced usage patterns with the real `AutoSwapprClient` that provides full Starknet integration.

**Features demonstrated:**
- ✅ Real contract parameter reading
- ✅ Token information retrieval (name, symbol, decimals)
- ✅ Token balance checking
- ✅ Token allowance checking
- ✅ USD price conversion
- ✅ Swap data creation
- ⚠️ Token approval (commented out for safety)
- ⚠️ Swap execution (commented out for safety)
- ✅ Advanced client features overview

**Prerequisites:**
- Set environment variables:
  ```bash
  export RPC_URL="https://starknet-mainnet.public.blastapi.io/rpc/v0_7"
  export PRIVATE_KEY="your_private_key_here"
  export ACCOUNT_ADDRESS="your_account_address_here"
  export CONTRACT_ADDRESS="0x05582ad635c43b4c14dbfa53cbde0df32266164a0d1b36e5b510e5b34aeb364b"
  ```

**Run the example:**
```bash
cargo run --example advanced_usage
```

## Environment Setup

Before running the examples, make sure you have:

1. **Rust installed** (latest stable version)
2. **Environment variables set** for real contract interactions
3. **Sufficient ETH/STRK** in your account for gas fees (for write operations)

## Safety Notes

- The examples include read-only operations that are safe to run
- Write operations (approvals, swaps) are commented out for safety
- Always test with small amounts first
- Make sure you understand the transaction costs before executing swaps

## Troubleshooting

### Common Issues

1. **"RPC URL NOT PROVIDED"**: Set the `RPC_URL` environment variable
2. **"PRIVATE KEY IS NOT PROVIDED"**: Set the `PRIVATE_KEY` environment variable
3. **"ACCOUNT ADDRESS NOT PROVIDED"**: Set the `ACCOUNT_ADDRESS` environment variable
4. **Connection errors**: Check your RPC URL and internet connection
5. **Transaction failures**: Ensure you have sufficient balance for gas fees

### Getting Help

If you encounter issues:
1. Check the error messages carefully
2. Verify your environment variables are set correctly
3. Ensure your account has sufficient balance
4. Check the Starknet network status
