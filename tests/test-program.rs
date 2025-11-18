//! Simple test program that uses pinocchio-utils

#![no_main]
use pinocchio::{
    account_info::AccountInfo, entrypoint, msg, pubkey::Pubkey, ProgramResult,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    msg!("Creating PDA with pinocchio-utils");

    let payer = &accounts[0];
    let pda = &accounts[1];

    // Get bump from instruction data (passed from test)
    let bump = data[0];

    // Seeds for PDA derivation
    let seeds = [b"test".as_ref()];

    // Call our utility! N=2 because we have 1 seed + 1 bump
    pinocchio_utils::create_pda_account::<2>(payer, pda, program_id, 100, &seeds, bump)?;

    msg!("SUCCESS!");
    Ok(())
}
