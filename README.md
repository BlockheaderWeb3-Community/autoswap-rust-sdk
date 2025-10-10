# AutoSwappr Rust SDK

A Rust SDK for interacting with the AutoSwappr contract on Starknet.

## Features

- 🔄 Execute manual token swaps on Ekubo
- 🦀 Full Rust type safety
- ⚡ Async/await support with Tokio

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
autoswappr-sdk = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

All other dependencies (like `starknet`, `axum`, etc.) are automatically included.

## AutoSwappr Contract Address

```
0x05582ad635c43b4c14dbfa53cbde0df32266164a0d1b36e5b510e5b34aeb364b
```

## Quick Start

```rust
use autoswappr_sdk::{AutoSwappr, constant::{STRK, USDC}};

#[tokio::main]
async fn main() {
    // Initialize the SDK
    let mut swapper = AutoSwappr::config(
        "https://starknet-mainnet.public.blastapi.io".to_string(),
        "YOUR_ACCOUNT_ADDRESS".to_string(),
        "YOUR_PRIVATE_KEY".to_string(),
        "0x05582ad635c43b4c14dbfa53cbde0df32266164a0d1b36e5b510e5b34aeb364b".to_string(),
    ).unwrap();

    // Execute swap (1 STRK to USDC)
    let result = swapper.ekubo_manual_swap(*STRK, *USDC, 1).await;

    match result {
        Ok(response) => println!("Swap successful! Tx: {:?}", response.tx_hash),
        Err(error) => println!("Swap failed: {}", error.message),
    }
}
```

## API Reference

### `AutoSwappr::config`

Configure a new AutoSwappr instance.

```rust
pub fn config(
    rpc_url: String,
    account_address: String,
    private_key: String,
    contract_address: String,
) -> Result<AutoSwappr, Json<ErrorResponse>>
```

### `ekubo_manual_swap`

Execute a manual token swap.

```rust
pub async fn ekubo_manual_swap(
    &mut self,
    token0: Felt,
    token1: Felt,
    swap_amount: u128,
) -> Result<Json<SuccessResponse>, Json<ErrorResponse>>
```

**Parameters:**

- `token0`: Source token address
- `token1`: Destination token address
- `swap_amount`: Amount to swap (in token units, not smallest denomination)

**Returns:**

- `Ok(SuccessResponse)`: Contains transaction hash on success
- `Err(ErrorResponse)`: Contains error message on failure

## Available Token Addresses

```rust
use autoswappr_sdk::constant::{STRK, USDC, ETH};

// STRK token
*STRK

// USDC token
*USDC

// ETH token
*ETH
```

## Security Considerations

⚠️ **Never expose private keys in your code or version control**

1. Use environment variables for sensitive data
2. Keep private keys secure and encrypted
3. Test with small amounts first

## Example with Environment Variables

```rust
use std::env;

let rpc_url = env::var("STARKNET_RPC_URL").expect("STARKNET_RPC_URL not set");
let account_address = env::var("ACCOUNT_ADDRESS").expect("ACCOUNT_ADDRESS not set");
let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set");

let mut swapper = AutoSwappr::config(
    rpc_url,
    account_address,
    private_key,
    "0x05582ad635c43b4c14dbfa53cbde0df32266164a0d1b36e5b510e5b34aeb364b".to_string(),
).unwrap();
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## Support

For support and questions, please open an issue on GitHub.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Related Projects

- [AutoSwappr TypeScript SDK](https://github.com/BlockheaderWeb3-Community/autoswap-sdk) - Original TypeScript implementation
