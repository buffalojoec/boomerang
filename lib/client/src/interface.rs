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

pub struct BoomerangTestClientConfig {
    pub advance_slot_hashes: Vec<Slot>,
    pub features_disabled: Vec<Pubkey>,
    pub program_file: String,
    pub program_id: Pubkey,
    pub rpc_commitment: CommitmentConfig,
    pub rpc_endpoint: String,
}
impl Default for BoomerangTestClientConfig {
    fn default() -> Self {
        Self {
            advance_slot_hashes: vec![],
            features_disabled: vec![],
            program_file: "program.so".to_string(),
            program_id: Pubkey::new_unique(),
            rpc_commitment: CommitmentConfig::processed(),
            rpc_endpoint: "http://127.0.0.1:8899".to_string(),
        }
    }
}

/// A client for testing programs
#[async_trait]
pub trait BoomerangTestClient {
    /// Create a new client
    async fn setup(config: &BoomerangTestClientConfig) -> Self;

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

    /// Create a transaction with the provided instructions, fee payer,
    /// signers, and recent blockhash
    fn create_transaction(
        &self,
        instructions: &[Instruction],
        fee_payer: &Keypair,
        signers: &[&Keypair],
        recent_blockhash: Hash,
    ) -> Transaction {
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
        instructions: &[Instruction],
        additional_signers: &[&Keypair],
    ) -> Transaction {
        let fee_payer = self.fee_payer();
        let recent_blockhash = self.last_blockhash();
        let mut signers = vec![&fee_payer];
        signers.extend(additional_signers);
        self.create_transaction(instructions, &fee_payer, &signers, recent_blockhash)
    }

    /// Create a default transaction with a new latest blockhash
    async fn create_default_transaction_with_new_blockhash(
        &mut self,
        instructions: &[Instruction],
        additional_signers: &[&Keypair],
    ) -> Transaction {
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
