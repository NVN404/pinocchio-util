# pinocchio-toolkit

[![Crates.io](https://img.shields.io/crates/v/pinocchio-toolkit.svg)](https://crates.io/crates/pinocchio-toolkit)
[![Docs.rs](https://docs.rs/pinocchio-toolkit/badge.svg)](https://docs.rs/pinocchio-toolkit)
[![License](https://img.shields.io/crates/l/pinocchio-toolkit.svg)](https://github.com/NVN404/pinocchio-util#license)

> **Zero-boilerplate helpers for Pinocchio Solana programs**


## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
pinocchio-toolkit = "0.1.0"
pinocchio = "0.9"
pinocchio-pubkey = "0.3"
```

## Usage

### Create a PDA Account

```rust
use pinocchio::account_info::AccountInfo;
use pinocchio_toolkit::create_pda_account;

// In your instruction handler:
pub fn create_vault(
    payer: &AccountInfo,
    vault: &AccountInfo,
    program_id: &[u8; 32],
) -> pinocchio::ProgramResult {
    // Derive PDA
    let seeds = [b"vault", payer.key()];
    let (vault_key, bump) = pinocchio_pubkey::find_program_address(&seeds, program_id);
    
    // Create the account with one line!
    create_pda_account::<2>(
        payer,
        vault,
        program_id,
        100,  // space in bytes
        &seeds,
        bump,
    )?;
    
    Ok(())
}
```

### What it does for you

Without `pinocchio-util`:
```rust
// 30+ lines of boilerplate:
// - Get rent sysvar
// - Build seed arrays with MaybeUninit
// - Construct CreateAccount instruction
// - Convert seeds to Seed types with unsafe
// - Call invoke_signed with correct types
```

With `pinocchio-toolkit`:
```rust
// One line:
create_pda_account::<2>(payer, pda, program_id, space, &seeds, bump)?;
```

##  Reference

### `create_pda_account<const N: usize>`

**Create a rent-exempt PDA account owned by your program.**  

**Arguments:**
- `payer: &AccountInfo` – The account paying for rent (must be signer)
- `pda: &AccountInfo` – The uninitialized PDA account
- `program_id: &Pubkey` – Your program ID
- `space: u64` – How many bytes you want (e.g. `core::mem::size_of::<MyData>()`)
- `seeds: &[&[u8]]` – PDA seeds WITHOUT the bump
- `bump: u8` – The PDA bump seed

**Const Generic:**
- `N: usize` – Maximum number of seed slices (must be >= `seeds.len()`)

**Returns:**
- `Ok(())` on success
- `Err(ProgramError)` on failure

**Errors:**
- `MissingRequiredSignature` - Payer is not a signer
- `AccountAlreadyInitialized` - PDA already has lamports
- `InvalidSeeds` - PDA derivation doesn't match
- `InvalidArgument` - Too many seeds for array size `N`

## Examples

### Real Working Program

See [`tests/test-program.rs`](tests/test-program.rs) for a complete example Solana program that uses `pinocchio-toolkit`:

```rust
use pinocchio::{account_info::AccountInfo, entrypoint, pubkey::Pubkey, ProgramResult};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let payer = &accounts[0];
    let pda = &accounts[1];
    let bump = data[0]; // Bump seed passed from client
    
    let seeds = [b"test".as_ref()];
    
    // One line to create PDA! 
    pinocchio_toolkit::create_pda_account::<2>(payer, pda, program_id, 100, &seeds, bump)?;
    
    Ok(())
}
```

**This real program:**
- ✅ Compiles with `cargo build-sbf`
- ✅ Deploys to Solana (via litesvm)
- ✅ Creates PDAs with only **1,930 compute units**
- ✅ Handles rent, validation, and CPI automatically

### Integration Tests

See [`tests/integration_test.rs`](tests/integration_test.rs) for the full test suite using litesvm.

**Run the tests:**
```bash
# Build the test program
cd tests/test-program && cargo build-sbf && cd ../..

# Run integration tests
cargo test --test integration_test -- --nocapture
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.
