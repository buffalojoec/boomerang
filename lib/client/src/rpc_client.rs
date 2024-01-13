use {
    crate::interface::{BoomerangTestClient, BoomerangTestClientConfig},
    async_trait::async_trait,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        account::Account,
        hash::Hash,
        pubkey::Pubkey,
        signature::Keypair,
        transaction::{Transaction, TransactionError},
    },
};

pub struct BoomerangRpcClient {
    fee_payer: Keypair,
    latest_blockhash: Hash,
    rpc_client: RpcClient,
}

#[async_trait]
impl BoomerangTestClient for BoomerangRpcClient {
    async fn setup(config: &BoomerangTestClientConfig) -> Self {
        let fee_payer = Keypair::new();
        let rpc_client =
            RpcClient::new_with_commitment(config.rpc_endpoint.clone(), config.rpc_commitment);
        let latest_blockhash = rpc_client.get_latest_blockhash().await.unwrap();
        Self {
            fee_payer,
            latest_blockhash,
            rpc_client,
        }
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
            .send_and_confirm_transaction(&transaction)
            .await
            .map(|_| ())
            .map_err(|err| err.get_transaction_error())
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
}
