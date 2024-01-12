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
        sysvar::{Sysvar, SysvarId},
        transaction::{Transaction, TransactionError},
    },
};

pub struct BoomerangClient {
    pub banks: BoomerangBanksClient,
    pub rpc: BoomerangRpcClient,
    pub use_banks: bool,
}

#[async_trait]
impl BoomerangTestClient for BoomerangClient {
    // TODO: No-op?
    async fn new(config: &BoomerangTestClientConfig) -> Self {
        let banks = BoomerangBanksClient::new(config).await;
        let rpc = BoomerangRpcClient::new(config).await;
        BoomerangClient {
            banks,
            rpc,
            use_banks: true,
        }
    }

    fn fee_payer(&self) -> Keypair {
        if self.use_banks {
            self.banks.fee_payer()
        } else {
            self.rpc.fee_payer()
        }
    }

    fn last_blockhash(&self) -> Hash {
        if self.use_banks {
            self.banks.last_blockhash()
        } else {
            self.rpc.last_blockhash()
        }
    }

    async fn new_latest_blockhash(&self) -> Hash {
        if self.use_banks {
            self.banks.new_latest_blockhash().await
        } else {
            self.rpc.new_latest_blockhash().await
        }
    }

    async fn process_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<Signature, Option<TransactionError>> {
        if self.use_banks {
            self.banks.process_transaction(transaction).await
        } else {
            self.rpc.process_transaction(transaction).await
        }
    }

    async fn get_account(
        &self,
        pubkey: &Pubkey,
    ) -> Result<Option<Account>, Box<dyn std::error::Error>> {
        if self.use_banks {
            self.banks.get_account(pubkey).await
        } else {
            self.rpc.get_account(pubkey).await
        }
    }

    fn set_sysvar<T: SysvarId + Sysvar>(&self, sysvar: &T) {
        if self.use_banks {
            self.banks.set_sysvar(sysvar)
        } else {
            self.rpc.set_sysvar(sysvar)
        }
    }
}
