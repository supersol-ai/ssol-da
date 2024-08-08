use anchor_lang::prelude::*;

declare_id!("GceLnmiKY1RGmcki6USeSjP6CnvVLJRiq34nQUD6TD5t");

mod constant;
mod da;
mod error;
mod instructions;
mod common;

#[program]
pub mod da_program {
    
    use super::*;
    
    pub use super::instructions::*;
    pub use super::common::*;

    /// Initialize the blob storage.
    pub fn initialize<'info>(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    /// Remove blobs from storage.
    pub fn remove_blob<'info>(ctx: Context<RemoveBlob>, opt_hash: Option<Hash>) -> Result<()> {
        instructions::remove_blob(ctx, opt_hash)
    }

    //. Insert blobs into storage
    pub fn insert_blob<'info>(ctx: Context<InsertBlob>, bump: u8, blob: Blob) -> Result<()> {
        instructions::insert_blob(ctx, bump, blob)
    }
}
