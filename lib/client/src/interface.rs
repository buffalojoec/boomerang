use {
    async_trait::async_trait,
    solana_sdk::{
        account::{Account, ReadableAccount},
        commitment_config::CommitmentConfig,
        hash::Hash,
        instruction::{Instruction, InstructionError},
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
        slot_history::Slot,
        transaction::{Transaction, TransactionError},
    },
};

#[derive(Clone)]
pub struct BoomerangTestClientConfig {
    pub features_disabled: Vec<Pubkey>,
    pub program_file: String,
    pub program_id: Pubkey,
    pub rpc_commitment: CommitmentConfig,
    pub rpc_endpoint: String,
    pub slots_per_epoch: u64,
    pub warp_slot: Slot,
}
impl Default for BoomerangTestClientConfig {
    fn default() -> Self {
        Self {
            features_disabled: vec![],
            program_file: "program.so".to_string(),
            program_id: Pubkey::new_unique(),
            rpc_commitment: CommitmentConfig::processed(),
            rpc_endpoint: "http://127.0.0.1:8899".to_string(),
            slots_per_epoch: 300, // Arbitrarily small number for testing
            warp_slot: 0,
        }
    }
}

/// A client for testing programs
#[async_trait]
pub trait BoomerangTestClient {
    /// Get the program ID
    fn program_id(&self) -> Pubkey;

    /// Get the fee payer
    fn fee_payer(&self) -> Keypair;

    /// Get the last blockhash
    fn last_blockhash(&self) -> Hash;

    /// Get a new latest blockhash
    async fn new_latest_blockhash(&mut self) -> Hash;

    /// Process a transaction
    async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), Option<TransactionError>>;

    /// Confirm a transaction
    async fn confirm_transaction(
        &self,
        signature: &Signature,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Get an account
    async fn get_account(
        &mut self,
        pubkey: &Pubkey,
    ) -> Result<Option<Account>, Box<dyn std::error::Error>>;

    async fn poll_for_next_epoch(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn poll_slots(&self, num_slots: u64) -> Result<(), Box<dyn std::error::Error>>;

    /// Create a transaction with the provided instructions, fee payer,
    /// signers, and recent blockhash
    fn create_transaction(
        &self,
        instructions: &mut [Instruction],
        fee_payer: &Keypair,
        signers: &[&Keypair],
        recent_blockhash: Hash,
    ) -> Transaction {
        // TODO: Hot-swapping the program ID at test runtime is necessary when
        // testing a program that gets deployed during genesis on the test
        // validator, such as a native program or an SPL Token program.
        // The test validator startup options for adding programs at genesis
        // will not overwrite the program account(s) at the existing address.
        // In order to eliminate this hot-swapping, we have to figure out a way
        // to load the program being tested at the proper address on test validator
        // startup.
        instructions.iter_mut().for_each(|ix| {
            ix.program_id = self.program_id();
        });
        Transaction::new_signed_with_payer(
            instructions,
            Some(&fee_payer.pubkey()),
            signers,
            recent_blockhash,
        )
    }

    /// Create a default transaction with the fee payer as the payer
    fn create_default_transaction(
        &self,
        instructions: &mut [Instruction],
        additional_signers: &[&Keypair],
    ) -> Transaction {
        // TODO: Hot-swapping the program ID at test runtime is necessary when
        // testing a program that gets deployed during genesis on the test
        // validator, such as a native program or an SPL Token program.
        // The test validator startup options for adding programs at genesis
        // will not overwrite the program account(s) at the existing address.
        // In order to eliminate this hot-swapping, we have to figure out a way
        // to load the program being tested at the proper address on test validator
        // startup.
        instructions.iter_mut().for_each(|ix| {
            ix.program_id = self.program_id();
        });
        let fee_payer = self.fee_payer();
        let recent_blockhash = self.last_blockhash();
        let mut signers = vec![&fee_payer];
        signers.extend(additional_signers);
        self.create_transaction(instructions, &fee_payer, &signers, recent_blockhash)
    }

    /// Create a default transaction with a new latest blockhash
    async fn create_default_transaction_with_new_blockhash(
        &mut self,
        instructions: &mut [Instruction],
        additional_signers: &[&Keypair],
    ) -> Transaction {
        // TODO: Hot-swapping the program ID at test runtime is necessary when
        // testing a program that gets deployed during genesis on the test
        // validator, such as a native program or an SPL Token program.
        // The test validator startup options for adding programs at genesis
        // will not overwrite the program account(s) at the existing address.
        // In order to eliminate this hot-swapping, we have to figure out a way
        // to load the program being tested at the proper address on test validator
        // startup.
        instructions.iter_mut().for_each(|ix| {
            ix.program_id = self.program_id();
        });
        let fee_payer = self.fee_payer();
        let recent_blockhash = self.new_latest_blockhash().await;
        let mut signers = vec![&fee_payer];
        signers.extend(additional_signers);
        self.create_transaction(instructions, &fee_payer, &signers, recent_blockhash)
    }

    /// Helper to validate a transaction succeeded
    async fn expect_successful_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), Option<TransactionError>> {
        self.process_transaction(transaction).await?;
        Ok(())
    }

    /// Helper to validate a transaction failed with a specific error
    async fn expect_failed_transaction(
        &mut self,
        transaction: Transaction,
        expected_err: TransactionError,
    ) {
        assert_eq!(
            self.process_transaction(transaction).await.unwrap_err(),
            Some(expected_err),
        );
    }

    /// Helper to validate a transaction failed with a specific
    /// `InstructionError`
    async fn expect_failed_transaction_instruction(
        &mut self,
        transaction: Transaction,
        index: u8,
        expected_err: InstructionError,
    ) {
        let result = self.process_transaction(transaction).await;
        match result {
            Ok(signature) => panic!("Transaction succeeded: {:#?}", signature),
            Err(err) => {
                if let Some(TransactionError::InstructionError(i, err)) = err {
                    assert_eq!(i, index);
                    assert_eq!(err, expected_err);
                } else {
                    panic!("Transaction failed with unknown error: {:#?}", err);
                }
            }
        }
    }

    /// Helper to validate an account's state matches the provided value
    async fn expect_account_state(
        &mut self,
        pubkey: &Pubkey,
        expected_state: &Account,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(account) = self.get_account(pubkey).await? {
            assert_eq!(account, *expected_state);
            Ok(())
        } else {
            panic!("Account not found: {}", pubkey);
        }
    }

    /// Helper to validate an account's data matches the provided bytes
    async fn expect_account_data(
        &mut self,
        pubkey: &Pubkey,
        expected_data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(account) = self.get_account(pubkey).await? {
            assert_eq!(account.data(), expected_data);
            Ok(())
        } else {
            panic!("Account not found: {}", pubkey);
        }
    }
}
