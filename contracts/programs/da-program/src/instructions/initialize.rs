use std::collections::BTreeMap;

use anchor_lang::prelude::*;

use crate::da::blob_storage::BlobStorage;

/// Initialize the blob storage.
pub fn initialize<'info>(ctx: Context<Initialize>) -> Result<()> {
    let blob_storage = &mut ctx.accounts.blob_storage;

    blob_storage.blobs = BTreeMap::new();

    Ok(())
}

/// Represents the accounts to be utilized during the initialization of the chunk accumulator..
///
/// WARNING: Simple 1 account model at the moment.
/// TODO: Migrate to mature account model.
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// The signer/payer for data storing transaction.
    #[account(mut)]
    pub owner: Signer<'info>,

    /// The account for storing blob data from disperser.
    ///
    /// Must be blank when init.
    /// Must be a signer.
    #[account(signer, zero)]
    pub blob_storage: Account<'info, BlobStorage>,

    /// Solana system program.
    pub system_system: Program<'info, System>,
}
