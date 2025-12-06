use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug, PartialEq)]
pub struct GlobalState {
    pub admin: [u8; 32],
    pub platform_fee_bps: [u8; 2],
    pub plaftorm_fee_account: [u8; 32],
    pub supported_tokens_mint: [u8; 32],
    pub bump: [u8; 1],
    pub initialized: [u8; 1],
}

impl GlobalState {
    pub const LEN: usize = core::mem::size_of::<GlobalState>();

    pub fn to_bytes(&self) -> Vec<u8> {
        bytemuck::bytes_of(self).to_vec()
    }

    pub fn better_to_bytes(&self) -> Vec<u8> {
        self.to_bytes()
    }
}
