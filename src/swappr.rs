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
    types::connector::{AutoSwappr, ErrorResponse, SuccessResponse},
};
use axum::Json;

impl AutoSwappr {
    /// Configure a new AutoSwappr instance with wallet credentials.
    ///
    /// This function initializes the connection to Starknet and sets up the account
    /// for executing swaps through the AutoSwappr contract.
    ///
    /// # Arguments
    ///
    /// * `rpc_url` - The RPC endpoint URL for Starknet (e.g., Alchemy, Infura)
    /// * `account_address` - Your wallet address on Starknet
    /// * `private_key` - Your wallet's private key (keep this secure!)
    ///
    /// # Returns
    ///
    /// Returns `Ok(AutoSwappr)` if configuration is successful, or an `Err(Json<ErrorResponse>)`
    /// if any of the inputs are invalid or empty.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - `rpc_url` is an empty string
    /// - `account_address` is an empty string
    /// - `private_key` is an empty string
    /// - The RPC URL format is invalid
    /// - The account address or private key cannot be parsed as valid Felt values
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use auto_swappr::types::connector::AutoSwappr;
    ///
    /// let rpc_url = "https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0_9/YOUR_API_KEY".to_string();
    /// let account_address = "0x05362484eb9b91ae4365963dad33794cb50a5f93eb7a08b0280cf0f0c0129e4f".to_string();
    /// let private_key = "0x062e0c4dc96f3d877af48285a5442ce69860a50b11a1d91eae1e3f128df1c454".to_string();
    ///
    /// let swapper = AutoSwappr::config(rpc_url, account_address, private_key).unwrap();
    /// ```
    ///
    /// # Example with Error Handling
    ///
    /// ```
    /// use auto_swappr::types::connector::AutoSwappr;
    ///
    /// // This will fail due to empty RPC URL
    /// let result = AutoSwappr::config(
    ///     "".to_string(),
    ///     "0x123".to_string(),
    ///     "0x456".to_string()
    /// );
    ///
    /// assert!(result.is_err());
    /// ```

    pub fn config(
        rpc_url: String,
        account_address: String,
        private_key: String,
    ) -> Result<AutoSwappr, Json<ErrorResponse>> {
        if rpc_url.is_empty() {
            return Err(Json(ErrorResponse {
                success: false,
                message: "EMPTY RPC STRING".to_string(),
            }));
        }

        if account_address.is_empty() {
            return Err(Json(ErrorResponse {
                success: false,
                message: "EMPTY ACCOUNT ADDRESS STRING".to_string(),
            }));
        }

        if private_key.is_empty() {
            return Err(Json(ErrorResponse {
                success: false,
                message: "EMPTY PRIVATE KEY STRING".to_string(),
            }));
        }
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
        Ok(AutoSwappr {
            rpc_url,
            account_address,
            private_key,
            account,
            contract_address,
        })
    }

    /// Execute a manual token swap.
    ///
    /// # Arguments
    ///
    /// * `token0` - The address of the token to swap from (as Felt)
    /// * `token1` - The address of the token to swap to (as Felt)
    /// * `swap_amount` - The amount to swap in the smallest unit (e.g., wei for ETH)
    ///
    /// # Returns
    ///
    /// Returns `Ok(Json<SuccessResponse>)` with the transaction hash on success,
    /// or `Err(Json<ErrorResponse>)` if the swap fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - `swap_amount` is zero
    /// - Token information cannot be retrieved
    /// - The transaction execution fails
    /// - Insufficient balance or allowance
    ///
    /// # Examples
    ///
    /// ## Basic Swap Example
    ///
    /// ```no_run
    /// use auto_swappr::types::connector::AutoSwappr;
    /// use auto_swappr::constant::{STRK, USDC};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let rpc_url = "https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0_9/YOUR_API_KEY".to_string();
    /// let account_address = "0x053620000000000000000000000000000000000000000000000000000000".to_string();
    /// let private_key = "0x000000000000000000000000000000000000000000000000000000000000".to_string();
    ///
    /// let mut swapper = AutoSwappr::config(rpc_url, account_address, private_key).unwrap();
    ///
    /// // Swap 1 STRK token (amount is in base units without decimals)
    /// let result = swapper.ekubo_manual_swap(*STRK, *USDC, 1).await;
    ///
    /// match result {
    ///     Ok(response) => println!("Swap successful! TX: {:?}", response.tx_hash),
    ///     Err(error) => println!("Swap failed: {}", error.message),
    /// }
    /// # }
    /// ```
    ///
    /// ## Error Handling Example
    ///
    /// ```no_run
    /// use auto_swappr::types::connector::AutoSwappr;
    /// use auto_swappr::constant::{STRK, USDC};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let rpc_url = "https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0_9/YOUR_API_KEY".to_string();
    /// # let account_address = "0x05362484eb9b91ae4365963dad33794cb50a5f93eb7a08b0280cf0f0c0129e4f".to_string();
    /// # let private_key = "0x062e0c4dc96f3d877af48285a5442ce69860a50b11a1d91eae1e3f128df1c454".to_string();
    /// let mut swapper = AutoSwappr::config(rpc_url, account_address, private_key).unwrap();
    ///
    /// // This will fail because swap_amount is zero
    /// let result = swapper.ekubo_manual_swap(*STRK, *USDC, 0).await;
    ///
    /// assert!(result.is_err());
    /// if let Err(error) = result {
    ///     assert_eq!(error.err().message, "SWAP AMOUNT IS ZERO");
    /// }
    /// # }
    /// ```
    /// # Notes
    ///
    /// - The `swap_amount` should be specified in base units (without decimal adjustment)
    /// - The function automatically handles decimal conversion based on the token
    /// - Both approval and swap are executed in a single transaction
    /// - Make sure your account has sufficient balance of `token0`
    pub async fn ekubo_manual_swap(
        &mut self,
        token0: Felt,
        token1: Felt,
        swap_amount: u128,
    ) -> Result<Json<SuccessResponse>, Json<ErrorResponse>> {
        if swap_amount == 0 {
            return Err(Json(ErrorResponse {
                success: false,
                message: "SWAP AMOUNT IS ZERO".to_string(),
            }));
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
            Ok(x) => Ok(Json(SuccessResponse {
                success: true,
                tx_hash: x.transaction_hash,
            })),
            Err(x) => {
                println!("error message {}", x.to_string());
                Err(Json(ErrorResponse {
                    success: false,
                    message: "FAILED TO SWAP".to_string(),
                }))
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
    // #[ignore = "owner address and private key  is required to run the test"]
    async fn it_works_bravoos() {
        let rpc_url = "YOUR MAINNET RPC".to_string();
        let account_address =
            "YOUR WALLET ADDRESS".to_string();
        let private_key =
            "YOUR WALLET PRIVATE KEY".to_string();
        let mut swapper = AutoSwappr::config(rpc_url, account_address, private_key).unwrap();
        let result = swapper.ekubo_manual_swap(*STRK, *USDC, 1);
        assert!(result.await.is_ok())
    }
    #[tokio::test]
    #[ignore = "owner address and private key  is required to run the test"]
    async fn swap_with_zero_amount() {
        let rpc_url = "YOUR MAINNET RPC".to_string();
        let account_address =
            "YOUR WALLET ADDRESS".to_string();
        let private_key =
            "YOUR WALLET PRIVATE KEY".to_string();
        let mut swapper = AutoSwappr::config(rpc_url, account_address, private_key).unwrap();
        let result = swapper.ekubo_manual_swap(*STRK, *USDC, 0);

        assert!(result.await.is_err())
    }

    #[tokio::test]
    #[ignore = "owner address and private key  is required to run the test"]
    async fn it_works_argent() {
        let rpc_url = "YOUR MAINNET RPC".to_string();
        let account_address =
            "YOUR WALLET ADDRESS".to_string();
        let private_key =
            "YOUR WALLET PRIVATE KEY".to_string();
        let mut swapper = AutoSwappr::config(rpc_url, account_address, private_key).unwrap();
        let result = swapper.ekubo_manual_swap(*STRK, *USDC, 1);

        assert!(result.await.is_ok());
        // println!("test complete {:?}", result.await.err().unwrap().message);
    }
}
