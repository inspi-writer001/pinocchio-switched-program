pub mod admin_withdraw;
pub mod create_streamer;
pub mod initialize;
pub mod tip_user;
pub mod withdraw;

pub use admin_withdraw::*;
pub use create_streamer::*;
pub use initialize::*;
pub use tip_user::*;
pub use withdraw::*;

#[repr(u8)]
pub enum SwitchedInstruction {
    Initialize,
    CreateStreamer,
    TipUser,
    Withdraw,
    AdminWithdrawFees,
    AdminWithdrawFeesAll,
}

impl TryFrom<&u8> for SwitchedInstruction {
    type Error = pinocchio::program_error::ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SwitchedInstruction::Initialize),
            1 => Ok(SwitchedInstruction::CreateStreamer),
            2 => Ok(SwitchedInstruction::TipUser),
            3 => Ok(SwitchedInstruction::Withdraw),
            4 => Ok(SwitchedInstruction::AdminWithdrawFees),
            5 => Ok(SwitchedInstruction::AdminWithdrawFeesAll),
            _ => Err(pinocchio::program_error::ProgramError::InvalidInstructionData),
        }
    }
}

#[macro_export]
macro_rules! require {
    ($condition:expr) => {
        if !$condition {
            return Err(pinocchio::program_error::ProgramError::InvalidArgument);
        }
    };
    ($condition:expr, $msg:expr) => {
        if !$condition {
            pinocchio_log::log!($msg);
            return Err(pinocchio::program_error::ProgramError::InvalidArgument);
        }
    };
}
