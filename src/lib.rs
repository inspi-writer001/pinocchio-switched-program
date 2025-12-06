use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

use crate::instruction::{process_create_streamer, process_intialize, SwitchedInstruction};

mod instruction;
mod state;
mod test;

entrypoint!(process_instruction);
pinocchio_pubkey::declare_id!("E9j22LsobSzd7D9trJ8hrE1tSLY3wP7AzJZX5DWnPY3y");

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
        SwitchedInstruction::CreateStreamer => process_create_streamer(accounts, data)?,
        _ => return Err(ProgramError::InvalidInstructionData),
    }
    Ok(())
}
