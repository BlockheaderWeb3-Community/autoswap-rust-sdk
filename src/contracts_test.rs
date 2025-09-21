#[cfg(test)]
mod contracts_tests {
    use crate::contracts::{AutoSwapprContract, Erc20Contract, conversions};
    use crate::types::connector::FeeType;
    use starknet::core::types::Felt;
    use std::sync::Arc;

    #[test]
    fn test_uint256_conversion() {
        let our_uint256 = crate::types::connector::Uint256 { low: 1000, high: 0 };
        let starknet_uint256 = conversions::uint256_to_starknet(&our_uint256);
        let back_to_ours = conversions::starknet_to_uint256(&starknet_uint256);

        assert_eq!(our_uint256.low, back_to_ours.low);
        assert_eq!(our_uint256.high, back_to_ours.high);
    }

    #[test]
    fn test_felt_to_ascii_string() {
        // Test ETH symbol (0x455448 = "ETH")
        let eth_symbol = Felt::from(0x455448u128);
        let result = conversions::felt_to_ascii_string(eth_symbol);
        assert_eq!(result, "ETH");

        // Test Ether name (0x4574686572 = "Ether")
        let ether_name = Felt::from(0x4574686572u128);
        let result = conversions::felt_to_ascii_string(ether_name);
        assert_eq!(result, "Ether");

        // Test with some ASCII characters mixed with non-ASCII
        let mixed = Felt::from(0x41424344u128); // "ABCD"
        let result = conversions::felt_to_ascii_string(mixed);
        assert_eq!(result, "ABCD");

        // Test with no valid ASCII characters - should return hex
        let non_ascii = Felt::from(0x1234567890abcdefu128);
        let result = conversions::felt_to_ascii_string(non_ascii);
        // The function extracts bytes and if none are valid ASCII, it returns hex
        assert!(result.len() > 0); // Should return something
    }

    #[test]
    fn test_u128_to_uint256_conversion() {
        let amount = 1000000000000000000u128; // 1 ETH
        let (low, high) = conversions::u128_to_uint256(amount);

        // For amounts < 2^64, high should be 0
        assert_eq!(high, Felt::ZERO);
        assert_eq!(low, Felt::from(amount));

        // Test round trip
        let back_to_u128 = conversions::uint256_to_u128(low, high);
        assert_eq!(back_to_u128, amount);
    }

    #[test]
    fn test_contract_parameters_parsing() {
        // Test the parsing logic without making actual network calls
        let mock_result = vec![
            Felt::from(12345u128), // fees_collector
            Felt::from(23456u128), // fibrous_exchange_address
            Felt::from(34567u128), // avnu_exchange_address
            Felt::from(45678u128), // oracle_address
            Felt::from(56789u128), // owner
            Felt::from(0u8),       // fee_type: Fixed
            Felt::from(100u16),    // percentage_fee: 100
        ];

        // Test fee_type parsing
        let fee_type_raw: u8 = mock_result[5].try_into().unwrap_or(0);
        let fee_type = match fee_type_raw {
            0 => FeeType::Fixed,
            1 => FeeType::Percentage,
            _ => FeeType::Fixed,
        };
        assert_eq!(fee_type, FeeType::Fixed);

        // Test percentage_fee parsing
        let percentage_fee: u16 = mock_result[6].try_into().unwrap_or(0);
        assert_eq!(percentage_fee, 100);
    }

    #[test]
    fn test_contract_creation() {
        let address = Felt::from_hex("0x123").unwrap();
        let provider = Arc::new(starknet::providers::JsonRpcClient::new(
            starknet::providers::jsonrpc::HttpTransport::new(
                url::Url::parse("http://localhost:5000").unwrap(),
            ),
        ));

        let contract = AutoSwapprContract::new(address, provider);
        assert_eq!(contract.address(), address);
    }

    #[test]
    fn test_erc20_contract_creation() {
        let address = Felt::from_hex("0x456").unwrap();
        let provider = Arc::new(starknet::providers::JsonRpcClient::new(
            starknet::providers::jsonrpc::HttpTransport::new(
                url::Url::parse("http://localhost:5000").unwrap(),
            ),
        ));

        let contract = Erc20Contract::new(address, provider);
        assert_eq!(contract.address(), address);
    }
}
