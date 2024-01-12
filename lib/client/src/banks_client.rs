use {
    crate::interface::{BoomerangTestClient, BoomerangTestClientConfig},
    async_trait::async_trait,
    solana_program_test::{BanksClient, ProgramTest, ProgramTestBanksClientExt},
    solana_sdk::{
        account::Account,
        hash::Hash,
        pubkey::Pubkey,
        signature::Keypair,
        sysvar::{Sysvar, SysvarId},
        transaction::{Transaction, TransactionError},
    },
};

pub struct BoomerangBanksClient {
    banks_client: BanksClient,
    fee_payer: Keypair,
    latest_blockhash: Hash,
}

#[async_trait]
impl BoomerangTestClient for BoomerangBanksClient {
    async fn new(config: &BoomerangTestClientConfig) -> Self {
        let mut program_test = ProgramTest::new(&config.program_file, config.program_id, None);
        config.features_disabled.iter().for_each(|feature| {
            program_test.deactivate_feature(*feature);
        });
        let context = program_test.start_with_context().await;
        let banks_client = context.banks_client;
        let fee_payer = context.payer;
        let latest_blockhash = context.last_blockhash;
        Self {
            banks_client,
            fee_payer,
            latest_blockhash,
        }
    }

    fn fee_payer(&self) -> Keypair {
        self.fee_payer.insecure_clone()
    }

    fn last_blockhash(&self) -> Hash {
        self.latest_blockhash
    }

    async fn new_latest_blockhash(&mut self) -> Hash {
        self.banks_client
            .get_new_latest_blockhash(&self.latest_blockhash)
            .await
            .unwrap()
    }

    async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), Option<TransactionError>> {
        self.banks_client
            .process_transaction(transaction)
            .await
            .map_err(|_| None)
    }

    async fn get_account(
        &mut self,
        pubkey: &Pubkey,
    ) -> Result<Option<Account>, Box<dyn std::error::Error>> {
        self.banks_client
            .get_account(*pubkey)
            .await
            .map_err(|err| err.into())
    }

    fn set_sysvar<T: SysvarId + Sysvar>(&self, _sysvar: &T) {
        unimplemented!()
    }
}
