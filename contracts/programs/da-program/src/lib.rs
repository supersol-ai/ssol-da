use anchor_lang::prelude::*;

declare_id!("GceLnmiKY1RGmcki6USeSjP6CnvVLJRiq34nQUD6TD5t");

mod constant;
mod da;
mod error;
mod instructions;

#[program]
pub mod da_program {
    use super::*;

    pub use super::error::*;
    pub use super::instructions::*;

    /// Initialize the blob storage.
    pub fn initialize<'info>(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    /// Remove blobs from storage.
    pub fn remove_blob<'info>(ctx: Context<RemoveBlob>, opt_hash: Option<[u8; 32]>) -> Result<()> {
        instructions::remove_blob(ctx, opt_hash)
    }
}
