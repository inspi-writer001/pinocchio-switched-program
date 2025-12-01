use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug, PartialEq)]
pub struct Streamer {
    pub user_wallet: [u8; 32],
    pub user_token_account: [u8; 32],
    pub bump: [u8; 1],
}

impl Streamer {
    pub const LEN: usize = core::mem::size_of::<Streamer>();

    pub fn to_bytes(&self) -> Vec<u8> {
        bytemuck::bytes_of(self).to_vec()
    }
}
