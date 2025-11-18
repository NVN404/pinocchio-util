// REAL INTEGRATION TEST - Actually testing create_pda_account with litesvm!

use litesvm::LiteSVM;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

// Include the compiled test program
// Build it with: cd tests/test-program && cargo build-sbf
const TEST_PROGRAM_SO: &[u8] = include_bytes!("test-program/target/deploy/test_program.so");

#[test]
fn test_create_pda_account_real() {
    println!("\n🔥 REAL TEST: Actually calling create_pda_account!");

    // Setup litesvm
    let mut svm = LiteSVM::new();

    // Create and fund payer
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10 * LAMPORTS_PER_SOL)
        .expect("Airdrop failed");
    println!("Payer funded: {}", payer.pubkey());

    // Deploy test program
    let program_id = Pubkey::new_unique();
    svm.add_program(program_id, TEST_PROGRAM_SO);
    println!("Test program deployed: {}", program_id);

    // Derive PDA
    let seeds: &[&[u8]] = &[b"test"];
    let (pda, bump) = Pubkey::find_program_address(seeds, &program_id);
    println!("PDA derived: {} (bump: {})", pda, bump);

    // Check PDA doesn't exist yet
    assert!(
        svm.get_account(&pda).is_none(),
        "PDA should not exist before test"
    );
    println!("PDA confirmed uninitialized");

    // Build instruction (pass bump in data)
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(pda, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        ],
        data: vec![bump], // Pass the bump seed
    };

    // Send transaction
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    println!("\nSending transaction...");
    let result = svm.send_transaction(tx);

    match result {
        Ok(meta) => {
            println!("Transaction successful!");
            println!("   Compute units: {:?}", meta.compute_units_consumed);

            // Verify PDA was created
            let pda_account = svm.get_account(&pda).expect("PDA should exist now!");

            println!("\nPDA Account Created:");
            println!("   Address: {}", pda);
            println!("   Lamports: {}", pda_account.lamports);
            println!("   Data length: {}", pda_account.data.len());
            println!("   Owner: {}", pda_account.owner);

            // Assertions
            assert!(pda_account.lamports > 0, "PDA should have rent-exempt lamports");
            assert_eq!(pda_account.data.len(), 100, "PDA should have 100 bytes");
            assert_eq!(pda_account.owner, program_id, "PDA should be owned by program");

            println!("\n ALL TESTS PASSED! ");
        }
        Err(e) => {
            panic!("\n❌ Transaction FAILED: {:?}", e);
        }
    }
}
