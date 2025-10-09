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
use reqwest::Client;
use serde_json::json;
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
    // Implemented: approve token and notify backend for auto-swap
    pub async fn ekubo_auto_swap(
        &mut self,
        token_from: Felt,
        token_to: Felt,
        amount: u128,
        backend_url: &str,
    ) -> Result<String, String> {
        if amount == 0 {
            return Err("ZERO SWAP AMOUNT".to_string());
        }

        // ensure token is supported to derive decimals
        let token_decimal = TokenAddress::new()
            .get_token_info_by_address(token_from)
            .map_err(|e| e.to_string())?
            .decimals;

        let actual_amount = amount * 10_u128.pow(token_decimal as u32);
        let (amount_low, amount_high) = u128_to_uint256(actual_amount);

        // Prepare approve call to allow contract to spend `token_from`
        let approve_call = Call {
            to: token_from,
            selector: selector!("approve"),
            calldata: vec![self.contract_address, amount_low, amount_high],
        };

        // set preconfirmed block for querying
        self.account
            .set_block_id(BlockId::Tag(BlockTag::PreConfirmed));

        // send approve transaction
        let approve_result = self
            .account
            .execute_v3(vec![approve_call])
            .send()
            .await
            .map_err(|e| format!("approve failed: {}", e))?;

        // Prepare payload for backend
        let payload = json!({
            "wallet_address": format!("0x{:x}", self.account.address()),
            "user_address": format!("0x{:x}", self.account.address()),
            "to_token": format!("0x{:x}", token_to),
            "from_token": format!("0x{:x}", token_from),
            "swap_amount": actual_amount.to_string(),
            "approve_tx_hash": format!("0x{:x}", approve_result.transaction_hash),
        });

        let client = Client::new();
        let resp = client
            .post(backend_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("network error: {}", e))?;

        let status = resp.status();
        let text = resp
            .text()
            .await
            .map_err(|e| format!("response read error: {}", e))?;

        if status.is_success() {
            Ok(text)
        } else {
            Err(format!("backend error: {} - {}", status, text))
        }
    }
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

    #[tokio::test]
    #[ignore = "owner address, private key and backend required to run the test"]
    async fn it_works_auto() {
        // This test exercises `ekubo_auto_swap` flow: approve + notify backend.
        // It is ignored by default because it requires a funded wallet and a reachable backend.
        let rpc_url = "YOUR MAINNET RPC".to_string();
        let account_address = "YOUR WALLET ADDRESS".to_string();
        let private_key = "YOUR WALLET PRIVATE KEY".to_string();
        let mut swapper = AutoSwappr::config(rpc_url, account_address, private_key);

        // Use STRK -> USDC for a tiny amount (1 unit). Backend URL is a placeholder and
        // should be replaced with a real auto-swapper endpoint when running the test.
        let backend_url = "https://example.com/api/auto-swap";
        let result = swapper.ekubo_auto_swap(*STRK, *USDC, 1, backend_url);

        // Print the result (Ok response body or Err description). The test is ignored
        // so it won't run in CI unless explicitly enabled.
        println!("auto swap test result: {:?}", result.await);
    }
}
