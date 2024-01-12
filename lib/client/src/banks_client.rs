use {
    crate::interface::{BoomerangTestClient, BoomerangTestClientConfig},
    async_trait::async_trait,
    solana_sdk::{
        account::AccountSharedData,
        hash::Hash,
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        sysvar::{Sysvar, SysvarId},
        transaction::{Transaction, TransactionError},
    },
};

pub struct BoomerangBanksClient;

#[async_trait]
impl BoomerangTestClient for BoomerangBanksClient {
    async fn new(_config: &BoomerangTestClientConfig) -> Self {
        BoomerangBanksClient
    }

    fn fee_payer(&self) -> Keypair {
        unimplemented!()
    }

    fn last_blockhash(&self) -> Hash {
        unimplemented!()
    }

    async fn new_latest_blockhash(&self) -> Hash {
        unimplemented!()
    }

    async fn process_transaction(
        &self,
        _transaction: Transaction,
    ) -> Result<Signature, TransactionError> {
        unimplemented!()
    }

    async fn get_account(
        &self,
        _pubkey: &Pubkey,
    ) -> Result<AccountSharedData, Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn set_sysvar<T: SysvarId + Sysvar>(&self, _sysvar: &T) {
        unimplemented!()
    }
}
