use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    pubkey::{self, pubkey_eq},
    sysvars::rent::Rent,
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::state::TokenAccount;

use crate::state::{GlobalState, Streamer};

pub fn process_create_streamer(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [signer, broadcaster, streamer_state, token_mint, streamer_ata, global_state, system_program, token_program, associated_token_program, rent_sysvar] =
        accounts
    else {
        return Err(pinocchio::program_error::ProgramError::NotEnoughAccountKeys);
    };

    // assert that signer is a signer
    assert!(signer.is_signer(), "Signer should be a signer");
    // assert broadcaster is a signer
    assert!(broadcaster.is_signer(), "Broadcaster should be a signer");
    // assert streamer_state is empty
    assert!(
        streamer_state.data_is_empty(),
        "Streamer state already exists"
    );
    // assert streamer_state derived computed is same as the one derived onchain
    let (derived_pda, bump) =
        pubkey::find_program_address(&[b"user", signer.key().as_ref()], &crate::ID);

    assert!(
        pubkey_eq(&derived_pda, streamer_state.key()),
        "Wrong keys for Derived Streamer state"
    );

    // assert token_mint is same as one stored in global_state
    let global_state_state =
        bytemuck::try_pod_read_unaligned::<GlobalState>(&global_state.try_borrow_data()?).unwrap();

    assert!(
        pubkey_eq(&global_state_state.supported_tokens_mint, token_mint.key()),
        "You provided wrong token mint"
    );

    // assert streamer_ata is not_empty
    assert!(!streamer_ata.data_is_empty(), "Streamer Ata does not exist");

    // assert streamer_ata is of type token_mint
    let streamer_ata_state = TokenAccount::from_account_info(streamer_ata)?;

    assert!(
        pubkey_eq(&streamer_ata_state.mint(), token_mint.key()),
        "Wrong Token Account for Streamer"
    );
    // assert streamer_ata authority is streamer_state
    assert!(
        pubkey_eq(streamer_ata_state.owner(), streamer_state.key()),
        "Streamer state must be authority"
    );

    let rent_state = Rent::from_account_info(rent_sysvar)?;
    // let rent_state = Rent::get()?;

    let mininum_balance = rent_state.minimum_balance(Streamer::LEN);

    let initial_bump = bump.to_le();
    let bump = [initial_bump];
    let seeds = [
        Seed::from(b"user"),
        Seed::from(signer.key()),
        Seed::from(&bump),
    ];

    let signers = Signer::from(&seeds);

    // create streamer_state
    CreateAccount {
        from: broadcaster,
        lamports: mininum_balance,
        owner: &crate::ID,
        space: Streamer::LEN as u64,
        to: streamer_state,
    }
    .invoke_signed(&[signers])?;

    // write to streamer_state
    let mut account_data = streamer_state.try_borrow_mut_data()?;
    let streamer_data = bytemuck::from_bytes_mut::<Streamer>(&mut account_data);

    streamer_data.user_token_account = *streamer_ata.key();
    streamer_data.user_wallet = *signer.key();
    streamer_data.bump = bump;
    Ok(())
}
