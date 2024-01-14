use {
    crate::interface::{BoomerangTestClient, BoomerangTestClientConfig},
    async_trait::async_trait,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        account::Account,
        commitment_config::CommitmentConfig,
        hash::Hash,
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
        transaction::{Transaction, TransactionError},
    },
};

pub struct BoomerangRpcClient {
    fee_payer: Keypair,
    latest_blockhash: Hash,
    program_id: Pubkey,
    rpc_client: RpcClient,
}

#[async_trait]
impl BoomerangTestClient for BoomerangRpcClient {
    async fn setup(config: &BoomerangTestClientConfig) -> Self {
        let fee_payer = Keypair::new();
        let program_id = config.program_id;

        let rpc_client =
            RpcClient::new_with_commitment(config.rpc_endpoint.clone(), config.rpc_commitment);
        let latest_blockhash = rpc_client.get_latest_blockhash().await.unwrap();

        let signature = rpc_client
            .request_airdrop(&fee_payer.pubkey(), 1000000000)
            .await
            .unwrap();
        rpc_client.confirm_transaction(&signature).await.unwrap();

        std::thread::sleep(std::time::Duration::from_secs(5));

        Self {
            fee_payer,
            latest_blockhash,
            program_id,
            rpc_client,
        }
    }

    fn program_id(&self) -> Pubkey {
        self.program_id
    }

    fn fee_payer(&self) -> Keypair {
        self.fee_payer.insecure_clone()
    }

    fn last_blockhash(&self) -> Hash {
        self.latest_blockhash
    }

    async fn new_latest_blockhash(&mut self) -> Hash {
        self.rpc_client.get_latest_blockhash().await.unwrap()
    }

    async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), Option<TransactionError>> {
        self.rpc_client
            .send_transaction(&transaction)
            .await
            .map(|_| ())
            .map_err(|err| err.get_transaction_error())
    }

    async fn confirm_transaction(
        &self,
        signature: &Signature,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            if self
                .rpc_client
                .confirm_transaction_with_commitment(signature, CommitmentConfig::finalized())
                .await?
                .value
            {
                return Ok(());
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    async fn get_account(
        &mut self,
        pubkey: &Pubkey,
    ) -> Result<Option<Account>, Box<dyn std::error::Error>> {
        self.rpc_client
            .get_account_with_commitment(pubkey, self.rpc_client.commitment())
            .await
            .map(|res| res.value)
            .map_err(|err| err.into())
    }

    async fn poll_for_next_epoch(&self) -> Result<(), Box<dyn std::error::Error>> {
        let epoch_info = self.rpc_client.get_epoch_info().await?;
        let current = epoch_info.epoch;
        loop {
            let epoch_info = self.rpc_client.get_epoch_info().await?;
            if epoch_info.epoch > current {
                return Ok(());
            }
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    }

    async fn poll_slots(&self, num_slots: u64) -> Result<(), Box<dyn std::error::Error>> {
        let slot = self.rpc_client.get_slot().await?;
        loop {
            let current_slot = self.rpc_client.get_slot().await?;
            if current_slot > slot + num_slots {
                return Ok(());
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
