use bytemuck::{Pod, Zeroable};
#[allow(unused_imports)]
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::state::{Mint, TokenAccount};

#[allow(unused_imports)]
use crate::state::{global_state, GlobalState};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug, PartialEq)]
pub struct InitializeSwitched {
    pub fee_bps: u16,
}

#[allow(unused)]
impl InitializeSwitched {
    pub fn to_bytes(&self) -> Vec<u8> {
        bytemuck::bytes_of(self).to_vec()
    }
}

pub fn process_intialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // fetch the accounts

    let [signer, global_state, token_account, token_mint, system_program, associated_token_program, token_program, rent_sysvar] =
        accounts
    else {
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
    };

    // let mut some_array = [0u8; 2];
    // some_array.copy_from_slice(&data[0..2]);
    // let ix_dataa: [u8; 2] = some_array;

    let ix_data = bytemuck::try_pod_read_unaligned::<InitializeSwitched>(data)
        .map_err(|_| pinocchio::program_error::ProgramError::InvalidInstructionData)?;

    // assert that signer is a signer
    assert!(signer.is_signer(), "Signer should be a Signer");

    // assert that the global_state is empty
    assert!(global_state.data_is_empty(), "Global State should be empty");

    // assert that we're using a 6 decimal token
    let token_mint_state = Mint::from_account_info(token_mint)?;
    assert!(
        token_mint_state.decimals() == 6,
        "only use a 6 decimal token"
    );

    // assert that token account is of the mint i.e token_mint == token_account.mint
    let token_account_state = TokenAccount::from_account_info(token_account)?;
    assert_eq!(
        token_account_state.mint(),
        token_mint.key(),
        "wrong token account for mint"
    );

    // assert that global_state computed offchain is same as the one deriving onchain
    let (derived_pda, bump) = pubkey::find_program_address(&[b"global_state"], &crate::ID);

    assert_eq!(
        &derived_pda,
        global_state.key(),
        "Global state does not match"
    );

    // calculate account rent
    let rent_state = Rent::from_account_info(rent_sysvar)?;
    // let rent_state = Rent::get()?;

    let mininum_balance = rent_state.minimum_balance(GlobalState::LEN);

    let initial_bump = bump.to_le();
    let bump = [initial_bump];
    let seeds = [Seed::from(b"global_state"), Seed::from(&bump)];

    let signers = Signer::from(&seeds);

    // create account
    CreateAccount {
        from: signer,
        lamports: mininum_balance,
        owner: &crate::ID,
        space: GlobalState::LEN as u64,
        to: global_state,
    }
    .invoke_signed(&[signers])?;

    // write mutably to account
    let state_data = &mut global_state.try_borrow_mut_data()?;

    let global_state_as_state_data = &mut bytemuck::from_bytes_mut::<GlobalState>(state_data);

    global_state_as_state_data.admin = *signer.key();
    global_state_as_state_data.bump = bump;
    global_state_as_state_data.initialized = [1u8; 1];
    global_state_as_state_data.plaftorm_fee_account = *token_account.key();
    global_state_as_state_data.platform_fee_bps = ix_data.fee_bps.to_le_bytes();
    // data[0..2];
    global_state_as_state_data.supported_tokens_mint = *token_mint.key();

    Ok(())
}
