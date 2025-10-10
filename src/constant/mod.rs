mod util;
use std::sync::LazyLock;

use starknet::core::types::Felt;
pub use util::u128_to_uint256;
//Token addresses for common tokens

pub static STRK: LazyLock<Felt> = LazyLock::new(|| {
    Felt::from_hex("0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d").unwrap()
});
pub static ETH: LazyLock<Felt> = LazyLock::new(|| {
    Felt::from_hex("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7").unwrap()
});
pub static USDC: LazyLock<Felt> = LazyLock::new(|| {
    Felt::from_hex("0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8").unwrap()
});
pub static USDT: LazyLock<Felt> = LazyLock::new(|| {
    Felt::from_hex("0x068f5c6a61780768455de69077e07e89787839bf8166decfbf92b645209c0fb8").unwrap()
});
pub static WBTC: LazyLock<Felt> = LazyLock::new(|| {
    Felt::from_hex("0x03fe2b97c1fd336e750087d68b9b867997fd64a2661ff3ca5a7c771641e8e7ac").unwrap()
});

#[allow(dead_code)]
#[derive(Clone)]
pub struct TokenAddress<'a> {
    pub tokens: Vec<TokenInfo<'a>>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
/// Token information for supported tokens
pub struct TokenInfo<'a> {
    pub address: Felt,
    pub symbol: &'a str,
    pub decimals: u8,
    name: &'a str,
}

impl Default for TokenAddress<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenAddress<'static> {
    pub fn new() -> Self {
        let tokens: Vec<TokenInfo> = vec![
            TokenInfo {
                address: *ETH,
                symbol: "ETH",
                decimals: 18,
                name: "Ether",
            },
            TokenInfo {
                address: *USDC,
                symbol: "USDC",
                decimals: 6,
                name: "USD Coin",
            },
            TokenInfo {
                address: *USDT,
                symbol: "USDT",
                decimals: 6,
                name: "Tether USD",
            },
            TokenInfo {
                address: *WBTC,
                symbol: "WBTC",
                decimals: 8,
                name: "Wrapped BTC",
            },
            TokenInfo {
                address: *STRK,
                symbol: "STRK",
                decimals: 18,
                name: "Starknet Token",
            },
        ];
        Self { tokens }
    }
    pub fn get_token_info(&self, address: &'static str) -> Result<TokenInfo<'static>, String> {
        let token = self
            .tokens
            .iter()
            .find(|x| x.symbol.to_lowercase() == address.to_lowercase())
            .cloned();
        match token {
            Some(x) => Ok(x),
            None => Err("TOKEN IS NOT AVAILABLE".to_string()),
        }
    }
    pub fn get_token_info_by_address(&self, address: Felt) -> Result<TokenInfo<'static>, String> {
        let token = self.tokens.iter().find(|x| x.address == address).cloned();
        match token {
            Some(x) => Ok(x),
            None => Err("TOKEN IS NOT AVAILABLE".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_success() {
        let strk = TokenAddress::new().get_token_info_by_address(*STRK);
        assert_eq!(strk.clone().unwrap().address, *STRK);
        println!("strk {:?} ", strk);
        let usdc = TokenAddress::new().get_token_info_by_address(*USDC);
        assert_eq!(usdc.unwrap().address, *USDC);
        let usdt = TokenAddress::new().get_token_info_by_address(*USDT);
        assert_eq!(usdt.unwrap().address, *USDT);
        let eth = TokenAddress::new().get_token_info_by_address(*ETH);
        assert_eq!(eth.unwrap().address, *ETH);
        let eth = TokenAddress::new().get_token_info_by_address(*ETH);
        assert_eq!(eth.unwrap().name, "Ether");
        let wbtc = TokenAddress::new().get_token_info_by_address(*WBTC);
        assert_eq!(wbtc.unwrap().address, *WBTC);
        let wbtc = TokenAddress::new().get_token_info_by_address(*WBTC);
        assert_eq!(wbtc.unwrap().decimals, 8);
    }

    #[test]
    #[should_panic(expected = "TOKEN IS NOT AVAILABLE")]
    fn should_panic() {
        let strk = TokenAddress::new().get_token_info("sol");
        assert_eq!(strk.unwrap().address, *STRK);
    }
}
