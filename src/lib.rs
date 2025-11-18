#![no_std]

//! Utilities for Pinocchio Solana programs.

use core::mem::MaybeUninit;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_pubkey::derive_address;
use pinocchio_system::instructions::CreateAccount;

pub fn create_pda_account<const N: usize>(
    payer: &AccountInfo,
    pda: &AccountInfo,
    program_id: &Pubkey,
    space: u64,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    // Basic checks
    if !payer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if pda.lamports() != 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if seeds.len() + 1 > N {
        return Err(ProgramError::InvalidArgument);
    }

    let bump_seed = [bump];
    let mut seeds_with_bump: [&[u8]; N] = [&[]; N];

    // Copy seeds using slice operations
    let seeds_count = seeds.len();
    seeds_with_bump[..seeds_count].copy_from_slice(seeds);
    seeds_with_bump[seeds_count] = &bump_seed;

    // Verify PDA derivation (None because bump is already in seeds_with_bump)
    let expected_pda = derive_address(&seeds_with_bump, None, program_id);
    if &expected_pda != pda.key() {
        return Err(ProgramError::InvalidSeeds);
    }

    // Calculate rent
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(space as usize);

    // Convert &[u8] slices to Seed types using MaybeUninit
    let mut pinocchio_seeds: [MaybeUninit<Seed>; N] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for i in 0..=seeds_count {
        pinocchio_seeds[i] = MaybeUninit::new(Seed::from(seeds_with_bump[i]));
    }

    let signer_seeds: &[Seed] = unsafe {
        core::slice::from_raw_parts(pinocchio_seeds.as_ptr() as *const Seed, seeds_count + 1)
    };

    // Create account using pinocchio-system CPI
    CreateAccount {
        from: payer,
        to: pda,
        lamports,
        space,
        owner: program_id,
    }
    .invoke_signed(&[Signer::from(signer_seeds)])?;

    Ok(())
}
