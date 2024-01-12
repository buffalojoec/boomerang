use {
    async_trait::async_trait,
    solana_sdk::{
        account::{AccountSharedData, ReadableAccount},
        hash::Hash,
        instruction::{Instruction, InstructionError},
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
        sysvar::{Sysvar, SysvarId},
        transaction::{Transaction, TransactionError},
    },
};

pub struct BoomerangTestClientConfig {
    pub features_disabled: Vec<Pubkey>,
    pub program_file: String,
    pub program_id: Pubkey,
}

/// A client for testing programs
#[async_trait]
pub trait BoomerangTestClient {
    /// Create a new client
    async fn new(config: &BoomerangTestClientConfig) -> Self;

    /// Get the fee payer
    fn fee_payer(&self) -> Keypair;

    /// Get the last blockhash
    fn last_blockhash(&self) -> Hash;

    /// Get a new latest blockhash
    async fn new_latest_blockhash(&self) -> Hash;

    /// Process a transaction
    async fn process_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<Signature, TransactionError>;

    /// Get an account
    async fn get_account(
        &self,
        pubkey: &Pubkey,
    ) -> Result<AccountSharedData, Box<dyn std::error::Error>>;

    /// Set a sysvar
    fn set_sysvar<T: SysvarId + Sysvar>(&self, sysvar: &T);

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
        &self,
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
        &self,
        transaction: Transaction,
    ) -> Result<Signature, TransactionError> {
        let result = self.process_transaction(transaction).await;
        if let Ok(signature) = result {
            Ok(signature)
        } else {
            panic!("Transaction failed: {}", result.unwrap_err());
        }
    }

    /// Helper to validate a transaction failed with a specific error
    async fn expect_failed_transaction(
        &self,
        transaction: Transaction,
        expected_err: TransactionError,
    ) {
        assert_eq!(
            self.process_transaction(transaction).await.unwrap_err(),
            expected_err,
        );
    }

    /// Helper to validate a transaction failed with a specific
    /// `InstructionError`
    async fn expect_failed_instruction(
        &self,
        transaction: Transaction,
        index: u8,
        expected_err: InstructionError,
    ) {
        assert_eq!(
            self.process_transaction(transaction).await.unwrap_err(),
            TransactionError::InstructionError(index, expected_err),
        );
    }

    /// Helper to validate an account's state matches the provided value
    async fn expect_account_state(
        &self,
        pubkey: &Pubkey,
        expected_state: &AccountSharedData,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let account = self.get_account(pubkey).await?;
        assert_eq!(account, *expected_state);
        Ok(())
    }

    /// Helper to validate an account's data matches the provided bytes
    async fn expect_account_data(
        &self,
        pubkey: &Pubkey,
        expected_data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let account = self.get_account(pubkey).await?;
        assert_eq!(account.data(), expected_data);
        Ok(())
    }
}
