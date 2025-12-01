use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

use crate::instruction::{process_intialize, SwitchedInstruction};

mod instruction;
mod state;

entrypoint!(process_instruction);
pinocchio_pubkey::declare_id!("27abzM8KfWuiYyiy6T3Dv1EeJWSPuBK7DDjtBQoapEfP");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match SwitchedInstruction::try_from(discriminator)? {
        SwitchedInstruction::Initialize => process_intialize(accounts, data)?,
        _ => return Err(ProgramError::InvalidInstructionData),
    }
    Ok(())
}
