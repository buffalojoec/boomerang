#![cfg(feature = "test-sbf")]

use {
    boomerang::client::{BoomerangClient, BoomerangTestClient},
    solana_address_lookup_table_program::state::{AddressLookupTable, LookupTableMeta},
    solana_program::{hash::Hash, instruction::InstructionError, slot_hashes::SlotHashes},
    solana_sdk::{
        address_lookup_table::instruction::{create_lookup_table, create_lookup_table_signed},
        clock::Slot,
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
    },
    std::borrow::Cow,
};

fn overwrite_slot_hashes_with_slots(client: &BoomerangClient, slots: &[Slot]) {
    let mut slot_hashes = SlotHashes::default();
    for slot in slots {
        slot_hashes.add(*slot, Hash::new_unique());
    }
    client.set_sysvar(&slot_hashes);
}

pub async fn test_create_lookup_table_idempotent(client: BoomerangClient) {
    let test_recent_slot = 123;
    overwrite_slot_hashes_with_slots(&client, &[test_recent_slot]);

    let authority_address = Pubkey::new_unique();
    let payer_pubkey = client.fee_payer().pubkey();

    let (create_lookup_table_ix, lookup_table_address) =
        create_lookup_table(authority_address, payer_pubkey, test_recent_slot);

    // First create should succeed
    {
        let transaction = client.create_default_transaction(&[create_lookup_table_ix.clone()], &[]);
        client
            .expect_successful_transaction(&transaction)
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
            .expect_successful_transaction(&transaction)
            .await
            .unwrap();
    }
}

pub async fn test_create_lookup_table_not_idempotent(client: BoomerangClient) {
    let test_recent_slot = 123;
    overwrite_slot_hashes_with_slots(&client, &[test_recent_slot]);

    let payer_pubkey = client.fee_payer().pubkey();
    let authority_keypair = Keypair::new();
    let authority_address = authority_keypair.pubkey();

    let (create_lookup_table_ix, ..) =
        create_lookup_table_signed(authority_address, payer_pubkey, test_recent_slot);

    let transaction =
        client.create_default_transaction(&[create_lookup_table_ix.clone()], &[&authority_keypair]);
    client
        .expect_successful_transaction(&transaction)
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
                &transaction,
                0,
                InstructionError::AccountAlreadyInitialized,
            )
            .await;
    }
}

pub async fn test_create_lookup_table_use_payer_as_authority(client: BoomerangClient) {
    let test_recent_slot = 123;
    overwrite_slot_hashes_with_slots(&client, &[test_recent_slot]);

    let payer_pubkey = client.fee_payer().pubkey();
    let authority_address = payer_pubkey;

    let (create_lookup_table_ix, ..) =
        create_lookup_table_signed(authority_address, payer_pubkey, test_recent_slot);

    let transaction = client.create_default_transaction(&[create_lookup_table_ix.clone()], &[]);
    client
        .expect_successful_transaction(&transaction)
        .await
        .unwrap();
}

pub async fn test_create_lookup_table_missing_signer(client: BoomerangClient) {
    let unsigned_authority_address = Pubkey::new_unique();

    let mut ix = create_lookup_table_signed(
        unsigned_authority_address,
        client.fee_payer().pubkey(),
        Slot::MAX,
    )
    .0;
    ix.accounts[1].is_signer = false;

    client
        .expect_failed_transaction_instruction(
            &client
                .create_default_transaction_with_new_blockhash(&[ix], &[])
                .await,
            0,
            InstructionError::MissingRequiredSignature,
        )
        .await;
}

pub async fn test_create_lookup_table_not_recent_slot(client: BoomerangClient) {
    let payer = client.fee_payer();
    let authority_address = Pubkey::new_unique();

    let ix = create_lookup_table(authority_address, payer.pubkey(), Slot::MAX).0;

    client
        .expect_failed_transaction_instruction(
            &client
                .create_default_transaction_with_new_blockhash(&[ix], &[])
                .await,
            0,
            InstructionError::InvalidInstructionData,
        )
        .await;
}

pub async fn test_create_lookup_table_pda_mismatch(client: BoomerangClient) {
    let test_recent_slot = 123;
    overwrite_slot_hashes_with_slots(&client, &[test_recent_slot]);

    let payer = client.fee_payer();
    let authority_address = Pubkey::new_unique();

    let mut ix = create_lookup_table(authority_address, payer.pubkey(), test_recent_slot).0;
    ix.accounts[0].pubkey = Pubkey::new_unique();

    client
        .expect_failed_transaction_instruction(
            &client
                .create_default_transaction_with_new_blockhash(&[ix], &[])
                .await,
            0,
            InstructionError::InvalidArgument,
        )
        .await;
}
