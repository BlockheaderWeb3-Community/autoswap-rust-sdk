use starknet::{
    accounts::{Account, ExecutionEncoding, SingleOwnerAccount},
    core::{
        chain_id,
        codec::Encode,
        types::{BlockId, BlockTag, Call, Felt},
    },
    macros::selector,
    providers::{JsonRpcClient, Url, jsonrpc::HttpTransport},
    signers::{LocalWallet, SigningKey},
};

use crate::{
    I129, PoolKey, SwapData, SwapParameters,
    constant::{TokenAddress, u128_to_uint256},
    types::connector::AutoSwappr,
};
#[allow(dead_code)]
type EkuboResponse = Result<
    starknet::core::types::InvokeTransactionResult,
    starknet::accounts::AccountError<
        starknet::accounts::single_owner::SignError<starknet::signers::local_wallet::SignError>,
    >,
>;
impl AutoSwappr {
    // wallet configuration
    pub fn config(rpc_url: String, account_address: String, private_key: String) -> AutoSwappr {
        let signer = LocalWallet::from(SigningKey::from_secret_scalar(
            Felt::from_hex(&private_key).unwrap(),
        ));
        let contract_address =
            Felt::from_hex("0x05582ad635c43b4c14dbfa53cbde0df32266164a0d1b36e5b510e5b34aeb364b")
                .unwrap();
        let address = Felt::from_hex(&account_address).unwrap();
        let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()));

        let account = SingleOwnerAccount::new(
            provider,
            signer,
            address,
            chain_id::MAINNET,
            ExecutionEncoding::New,
        );
        AutoSwappr {
            rpc_url,
            account_address,
            private_key,
            account,
            contract_address,
        }
    }

    // to do this function need a proper error handling
    // ekubo manual swap
    pub async fn ekubo_manual_swap(
        &mut self,
        token0: Felt,
        token1: Felt,
        swap_amount: u128,
    ) -> Result<String, String> {
        if swap_amount == 0 {
            return Err("ZERO SWAP AMOUNT".to_string());
        }

        let token_decimal = TokenAddress::new()
            .get_token_info_by_address(token0)
            .unwrap()
            .decimals;
        let actual_amount = swap_amount * 10_u128.pow(token_decimal as u32);
        let (amount_low, amount_high) = u128_to_uint256(actual_amount);

        let pool_key = PoolKey::new(token0, token1);
        let swap_parameters = SwapParameters::new(I129::new(actual_amount, false), false);
        let swap_data = SwapData::new(swap_parameters, pool_key, self.account.address());
        // let mut account  = self.account;
        self.account
            .set_block_id(BlockId::Tag(BlockTag::PreConfirmed));

        let mut serialized = vec![];
        swap_data.encode(&mut serialized).unwrap();

        let approve_call = Call {
            to: token0,
            selector: selector!("approve"),
            calldata: vec![self.contract_address, amount_low, amount_high],
        };
        println!("Calldata: {:?}", serialized);

        let swap_call = Call {
            to: self.contract_address,
            selector: selector!("ekubo_manual_swap"),
            calldata: serialized,
        };

        let result = self
            .account
            .execute_v3(vec![approve_call, swap_call])
            .send()
            .await;
        match result {
            Ok(x) => {
                println!("txt is succesful {:?}", x);
                return Ok("SUCCESSFUL".to_string());
            }
            Err(x) => {
                println!("error message {}", x);
                Err("error ".to_string())
            }
        }
    }

    // pub async fn  ekubo_auto_swap(){
    //     // todo steph
    //     // approve contract to spend
    //     //  sent a post request to auto swapper backend
    //     // post request arg will look like  this
    //     // ======       wallet_address, user address
    //     //  =======      to_token,
    //     // =======     from_token,
    //     // =======      swap_amount,

    //     // below  is how the approval fall will look
    //     // handle error

    //     // let account = approver_signer_account();

    //     // if !is_valid_address(token) {
    //     //     return Err("INVALID TOKEN ADDRESS".to_string());
    //     // }

    //     // let spender = contract_address_felt();

    //     // let token = Felt::from_hex(token).expect("TOKEN ADDRESS NOT PROVIDED");

    //     // // Convert amount to uint256 (split into low and high parts)
    //     // let (amount_low, amount_high) = u128_to_uint256(amount);

    //     // // Prepare the calldata: [spender, amount_low, amount_high]
    //     // let calldata = vec![spender, amount_low, amount_high];

    //     // let call = Call {
    //     //     to: token,
    //     //     selector: get_selector_from_name("approve").unwrap(),
    //     //     calldata,
    //     // };
    //     // let execution = account
    //     //     .execute_v3(vec![call])
    //     //     .send()
    //     //     .await
    //     //     .map_err(|e| e.to_string())?;
    //     // Ok(execution.transaction_hash)
    // }
}

#[cfg(test)]
mod tests {
    use crate::constant::{STRK, USDC};

    use super::*;

    #[tokio::test]
    #[ignore = "owner address and private key  is required to run the test"]
    async fn it_works_bravoos() {
        let rpc_url = "YOUR MAINNET RPC".to_string();
        let account_address = "YOUR WALLET ADDRESS".to_string();
        let private_key = "YOUR WALLET PRIVATE KEY".to_string();
        let mut swapper = AutoSwappr::config(rpc_url, account_address, private_key);
        let result = swapper.ekubo_manual_swap(*STRK, *USDC, 1);

        // assert!(result.await.clone().is_ok());
        println!("test complete {:?}", result.await.ok());
    }

    #[tokio::test]
    #[ignore = "owner address and private key  is required to run the test"]
    async fn it_works_argent() {
        // currently having issue with argent wallet ()
        let rpc_url = "YOUR MAINNET RPC".to_string();
        let account_address = "YOUR WALLET ADDRESS".to_string();
        let private_key = "YOUR WALLET PRIVATE KEY".to_string();
        let mut swapper = AutoSwappr::config(rpc_url, account_address, private_key);
        let result = swapper.ekubo_manual_swap(*STRK, *USDC, 1);

        // assert!(result.await.clone().is_ok());
        println!("test complete {:?}", result.await.ok());
    }
}
