mod banks_client;
mod interface;
mod rpc_client;

pub use interface::{BoomerangTestClient, BoomerangTestClientConfig};
use {
    async_trait::async_trait,
    banks_client::BoomerangBanksClient,
    rpc_client::BoomerangRpcClient,
    solana_sdk::{
        account::Account,
        hash::Hash,
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        transaction::{Transaction, TransactionError},
    },
};

pub struct BoomerangClient {
    // I don't love this, btw.
    pub banks: Option<BoomerangBanksClient>,
    pub rpc: Option<BoomerangRpcClient>,
    pub use_banks: bool,
}
impl BoomerangClient {
    pub async fn new(config: &BoomerangTestClientConfig, use_banks: bool) -> Self {
        let (banks, rpc) = if use_banks {
            let banks = BoomerangBanksClient::setup(config).await;
            (Some(banks), None)
        } else {
            let rpc = BoomerangRpcClient::setup(config).await;
            (None, Some(rpc))
        };
        Self {
            banks,
            rpc,
            use_banks,
        }
    }
}

#[async_trait]
impl BoomerangTestClient for BoomerangClient {
    // This function is a no-op, for now.
    // The config parameters for each need to be split out.
    async fn setup(_config: &BoomerangTestClientConfig) -> Self {
        BoomerangClient {
            banks: None,
            rpc: None,
            use_banks: true,
        }
    }

    fn program_id(&self) -> Pubkey {
        if self.use_banks {
            self.banks.as_ref().unwrap().program_id()
        } else {
            self.rpc.as_ref().unwrap().program_id()
        }
    }

    fn fee_payer(&self) -> Keypair {
        if self.use_banks {
            // Ahh (!!)
            self.banks.as_ref().unwrap().fee_payer()
        } else {
            self.rpc.as_ref().unwrap().fee_payer()
        }
    }

    fn last_blockhash(&self) -> Hash {
        if self.use_banks {
            self.banks.as_ref().unwrap().last_blockhash()
        } else {
            self.rpc.as_ref().unwrap().last_blockhash()
        }
    }

    async fn new_latest_blockhash(&mut self) -> Hash {
        if self.use_banks {
            self.banks.as_mut().unwrap().new_latest_blockhash().await
        } else {
            self.rpc.as_mut().unwrap().new_latest_blockhash().await
        }
    }

    async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), Option<TransactionError>> {
        if self.use_banks {
            self.banks
                .as_mut()
                .unwrap()
                .process_transaction(transaction)
                .await
        } else {
            self.rpc
                .as_mut()
                .unwrap()
                .process_transaction(transaction)
                .await
        }
    }

    async fn confirm_transaction(
        &self,
        signature: &Signature,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.use_banks {
            self.banks
                .as_ref()
                .unwrap()
                .confirm_transaction(signature)
                .await
        } else {
            self.rpc
                .as_ref()
                .unwrap()
                .confirm_transaction(signature)
                .await
        }
    }

    async fn get_account(
        &mut self,
        pubkey: &Pubkey,
    ) -> Result<Option<Account>, Box<dyn std::error::Error>> {
        if self.use_banks {
            self.banks.as_mut().unwrap().get_account(pubkey).await
        } else {
            self.rpc.as_mut().unwrap().get_account(pubkey).await
        }
    }
}
