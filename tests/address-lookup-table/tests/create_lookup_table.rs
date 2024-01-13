#![cfg(feature = "test-sbf")]

use {
    boomerang::client::{BoomerangClient, BoomerangTestClient},
    solana_address_lookup_table_program::state::{AddressLookupTable, LookupTableMeta},
    solana_program::instruction::InstructionError,
    solana_sdk::{
        address_lookup_table::instruction::{create_lookup_table, create_lookup_table_signed},
        clock::Slot,
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
    },
    std::borrow::Cow,
};

pub const TEST_RECENT_SLOT: Slot = 123;

pub async fn test_create_lookup_table_idempotent(mut client: BoomerangClient) {
    let authority_address = Pubkey::new_unique();
    let payer_pubkey = client.fee_payer().pubkey();

    let (create_lookup_table_ix, lookup_table_address) =
        create_lookup_table(authority_address, payer_pubkey, TEST_RECENT_SLOT);

    // First create should succeed
    {
        let transaction = client.create_default_transaction(&[create_lookup_table_ix.clone()], &[]);
        client
            .expect_successful_transaction(transaction)
            .await
            .unwrap();

        let meta = LookupTableMeta {
            deactivation_slot: Slot::MAX,
            last_extended_slot: 0,
            last_extended_slot_start_index: 0,
            authority: Some(authority_address),
            _padding: 0,
        };
        let addresses: Cow<'_, [Pubkey]> = Cow::default();
        let check_lookup_table = AddressLookupTable { meta, addresses }
            .serialize_for_tests()
            .unwrap();
        client
            .expect_account_data(&lookup_table_address, &check_lookup_table)
            .await
            .unwrap();
    }

    // Second create should succeed too
    {
        let transaction = client
            .create_default_transaction_with_new_blockhash(&[create_lookup_table_ix], &[])
            .await;
        client
            .expect_successful_transaction(transaction)
            .await
            .unwrap();
    }
}

pub async fn test_create_lookup_table_not_idempotent(mut client: BoomerangClient) {
    let payer_pubkey = client.fee_payer().pubkey();
    let authority_keypair = Keypair::new();
    let authority_address = authority_keypair.pubkey();

    let (create_lookup_table_ix, ..) =
        create_lookup_table_signed(authority_address, payer_pubkey, TEST_RECENT_SLOT);

    let transaction =
        client.create_default_transaction(&[create_lookup_table_ix.clone()], &[&authority_keypair]);
    client
        .expect_successful_transaction(transaction)
        .await
        .unwrap();

    // Second create should fail
    {
        let transaction = client
            .create_default_transaction_with_new_blockhash(
                &[create_lookup_table_ix],
                &[&authority_keypair],
            )
            .await;
        client
            .expect_failed_transaction_instruction(
                transaction,
                0,
                InstructionError::AccountAlreadyInitialized,
            )
            .await;
    }
}

pub async fn test_create_lookup_table_use_payer_as_authority(mut client: BoomerangClient) {
    let payer_pubkey = client.fee_payer().pubkey();
    let authority_address = payer_pubkey;

    let (create_lookup_table_ix, ..) =
        create_lookup_table_signed(authority_address, payer_pubkey, TEST_RECENT_SLOT);

    let transaction = client.create_default_transaction(&[create_lookup_table_ix.clone()], &[]);
    client
        .expect_successful_transaction(transaction)
        .await
        .unwrap();
}

pub async fn test_create_lookup_table_missing_signer(mut client: BoomerangClient) {
    let unsigned_authority_address = Pubkey::new_unique();

    let mut ix = create_lookup_table_signed(
        unsigned_authority_address,
        client.fee_payer().pubkey(),
        Slot::MAX,
    )
    .0;
    ix.accounts[1].is_signer = false;

    let tx = client
        .create_default_transaction_with_new_blockhash(&[ix], &[])
        .await;
    client
        .expect_failed_transaction_instruction(tx, 0, InstructionError::MissingRequiredSignature)
        .await;
}

pub async fn test_create_lookup_table_not_recent_slot(mut client: BoomerangClient) {
    let payer = client.fee_payer();
    let authority_address = Pubkey::new_unique();

    let ix = create_lookup_table(authority_address, payer.pubkey(), Slot::MAX).0;

    let tx = client
        .create_default_transaction_with_new_blockhash(&[ix], &[])
        .await;
    client
        .expect_failed_transaction_instruction(tx, 0, InstructionError::InvalidInstructionData)
        .await;
}

pub async fn test_create_lookup_table_pda_mismatch(mut client: BoomerangClient) {
    let payer = client.fee_payer();
    let authority_address = Pubkey::new_unique();

    let mut ix = create_lookup_table(authority_address, payer.pubkey(), TEST_RECENT_SLOT).0;
    ix.accounts[0].pubkey = Pubkey::new_unique();

    let tx = client
        .create_default_transaction_with_new_blockhash(&[ix], &[])
        .await;
    client
        .expect_failed_transaction_instruction(tx, 0, InstructionError::InvalidArgument)
        .await;
}
