use starknet::core::types::Felt;

// Helper function to convert u128 to (low, high) felts for uint256
pub fn u128_to_uint256(amount: u128) -> (Felt, Felt) {
    let amount_low = Felt::from(amount & 0xFFFFFFFFFFFFFFFF); // Lower 64 bits (NOT 128!)
    let amount_high = Felt::from(amount >> 64); // Upper 64 bits
    (amount_low, amount_high)
}
