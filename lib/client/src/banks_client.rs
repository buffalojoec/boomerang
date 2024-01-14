use {
    crate::interface::{BoomerangTestClient, BoomerangTestClientConfig},
    async_trait::async_trait,
    solana_program_test::{ProgramTest, ProgramTestBanksClientExt, ProgramTestContext},
    solana_sdk::{
        account::Account,
        hash::Hash,
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        slot_hashes::SlotHashes,
        transaction::{Transaction, TransactionError},
    },
};

pub struct BoomerangBanksClient {
    program_id: Pubkey,
    program_test_context: ProgramTestContext,
}

impl BoomerangBanksClient {
    pub async fn setup(config: &BoomerangTestClientConfig) -> Self {
        let program_id = config.program_id;

        let mut program_test = ProgramTest::new(&config.program_file, config.program_id, None);
        config.features_disabled.iter().for_each(|feature| {
            program_test.deactivate_feature(*feature);
        });

        let program_test_context = program_test.start_with_context().await;
        if config.warp_slot > 0 {
            let mut slot_hashes = SlotHashes::default();
            slot_hashes.add(config.warp_slot, Hash::new_unique());
            program_test_context.set_sysvar(&slot_hashes);
        }

        Self {
            program_id,
            program_test_context,
        }
    }
}

#[async_trait]
impl BoomerangTestClient for BoomerangBanksClient {
    fn program_id(&self) -> Pubkey {
        self.program_id
    }

    fn fee_payer(&self) -> Keypair {
        self.program_test_context.payer.insecure_clone()
    }

    fn last_blockhash(&self) -> Hash {
        self.program_test_context.last_blockhash
    }

    async fn new_latest_blockhash(&mut self) -> Hash {
        self.program_test_context
            .banks_client
            .get_new_latest_blockhash(&self.program_test_context.last_blockhash)
            .await
            .unwrap()
    }

    async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), Option<TransactionError>> {
        self.program_test_context
            .banks_client
            .process_transaction(transaction)
            .await
            .map_err(|err| Some(err.unwrap()))
    }

    async fn poll_for_next_epoch(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!() // TODO: Implement for banks client
    }

    async fn poll_slots(&self, _num_slots: u64) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!() // TODO: Implement for banks client
    }

    async fn confirm_transaction(
        &self,
        _signature: &Signature,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn get_account(
        &mut self,
        pubkey: &Pubkey,
    ) -> Result<Option<Account>, Box<dyn std::error::Error>> {
        self.program_test_context
            .banks_client
            .get_account(*pubkey)
            .await
            .map_err(|err| err.into())
    }
}
