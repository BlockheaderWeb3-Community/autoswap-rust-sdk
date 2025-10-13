#[cfg(test)]
mod contracts_tests {
    use crate::types::connector::FeeType;
    use starknet::core::types::Felt;

    // Original tests
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

    // New tests added
    #[test]
    fn test_fee_type_enum() {
        // Test Fixed fee type
        assert_eq!(FeeType::Fixed.to_u8(), 0);
        assert_eq!(FeeType::from_u8(0), FeeType::Fixed);
        
        // Test Percentage fee type
        assert_eq!(FeeType::Percentage.to_u8(), 1);
        assert_eq!(FeeType::from_u8(1), FeeType::Percentage);
        
        // Test default for unknown value (returns Percentage for non-zero)
        assert_eq!(FeeType::from_u8(99), FeeType::Percentage);
    }

    #[test]
    fn test_pool_key_creation() {
        use crate::types::connector::PoolKey;
        
        let token0 = Felt::from_hex("0x123").unwrap();
        let token1 = Felt::from_hex("0x456").unwrap();
        
        let pool_key = PoolKey::new(token0, token1);
        
        assert_eq!(pool_key.token0, token0);
        assert_eq!(pool_key.token1, token1);
        assert_eq!(pool_key.extension, Felt::ZERO);
    }

    #[test]
    fn test_i129_struct() {
        use crate::types::connector::I129;
        
        let amount = I129::new(1000000, false);
        assert_eq!(amount.mag, 1000000);
        assert_eq!(amount.sign, false);
        
        let negative = I129::new(500000, true);
        assert_eq!(negative.mag, 500000);
        assert_eq!(negative.sign, true);
    }

    #[test]
    fn test_swap_parameters() {
        use crate::types::connector::{SwapParameters, I129};
        
        let amount = I129::new(1000000, false);
        let swap_params = SwapParameters::new(amount, false);
        
        assert_eq!(swap_params.amount.mag, 1000000);
        assert_eq!(swap_params.is_token1, false);
        assert_eq!(swap_params.skip_ahead, 0);
    }

    #[test]
    fn test_felt_creation() {
        let felt1 = Felt::from(12345u128);
        let felt2 = Felt::from_hex("0x123").unwrap();
        
        assert!(felt1 != felt2);
        assert_eq!(felt1, Felt::from(12345u128));
    }

    #[test]
    fn test_contract_info_parsing() {
        use crate::types::connector::ContractInfo;
        
        let info = ContractInfo {
            fees_collector: "0x123".to_string(),
            fibrous_exchange_address: "0x456".to_string(),
            avnu_exchange_address: "0x789".to_string(),
            oracle_address: "0xabc".to_string(),
            owner: "0xdef".to_string(),
            fee_type: FeeType::Fixed,
            percentage_fee: 100,
        };
        
        assert_eq!(info.fee_type, FeeType::Fixed);
        assert_eq!(info.percentage_fee, 100);
    }

    #[test]
    fn test_route_struct() {
        use crate::types::connector::Route;
        
        let route = Route {
            token_from: Felt::from_hex("0x123").unwrap(),
            token_to: Felt::from_hex("0x456").unwrap(),
            exchange_address: Felt::from_hex("0x789").unwrap(),
            percent: 100,
            additional_swap_params: vec![],
        };
        
        assert_eq!(route.percent, 100);
        assert_eq!(route.additional_swap_params.len(), 0);
    }
}
