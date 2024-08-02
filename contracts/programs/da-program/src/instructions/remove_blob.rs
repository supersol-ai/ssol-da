use std::collections::BTreeMap;

use anchor_lang::prelude::*;

use crate::da::blob_storage::BlobStorage;

/// Remove blobs from blob storage.
///
/// Remove a specific blob if its hash is given.
/// Remove all blobs in other case.
pub fn remove_blob<'info>(ctx: Context<RemoveBlob>, opt_hash: Option<[u8; 32]>) -> Result<()> {
    let blob_storage: &mut Account<BlobStorage> = &mut ctx.accounts.blob_storage;

    if let Some(hash) = opt_hash {
        blob_storage.blobs.remove(&hash);
    } else {
        blob_storage.blobs = BTreeMap::new();
    }

    Ok(())
}

/// Represents the accounts to be utilized during the clearing operation of the blob storage.
#[derive(Accounts)]
pub struct RemoveBlob<'info> {
    /// The signer who initiates the clearing operation.
    #[account(mut)]
    pub owner: Signer<'info>,

    /// The account for storing blob data from disperser.
    ///
    /// Must be a signer.
    /// Will be removed some amount of blob from this account.
    #[account(signer, mut)]
    pub blob_storage: Account<'info, BlobStorage>,

    /// Soalan system program.
    pub system_program: Program<'info, System>,
}
