use anchor_lang::prelude::*;

use crate::constant::BLOB_SIZE;
use crate::da::{blob_storage::BlobStorage, block_root::BlockRoot};
use crate::error::ErrorCode;
use crate::common::*;

/// Insert blob data on blob storage
///
/// If blob's status is completed, insert into blob storage.
pub fn insert_blob<'info>(ctx: Context<InsertBlob>, bump: u8, blob: Blob) -> Result<()> {
    if blob.body.len() > BLOB_SIZE as usize {
        return Err(error!(ErrorCode::BlobSizeTooLong));
    }

    let blob_storage = &mut ctx.accounts.blob_storage;
    let block_root = &mut ctx.accounts.block_root;
    let current_slot = ctx.accounts.clock.slot;

    let hash = blob.hash.clone();

    blob_storage.update_merkle_tree(blob);

    if let Some(merkle_root) = blob_storage.get_merkle_root(&hash) {
        msg!(
            "entire rollup with hash: {:?} are completed with root {:?}",
            hash,
            merkle_root,
        );

        block_root.update_root(&merkle_root, current_slot);

        msg!(
            "block root for slot {}, blob root: {:?}, combined root: {:?}",
            current_slot,
            merkle_root,
            block_root.hash,
        );

        blob_storage.remove(&hash);
    }
    
    Ok(())
}

/// Represents the accounts to be utilized during the insertion operation of the blob storage.
#[derive(Accounts)]
pub struct InsertBlob<'info> {
    /// The signer who initiates the insertion operation.
    #[account(mut)]
    pub owner: Signer<'info>,

    /// The account for storing blob data from disperser.
    ///
    /// Must be a signer.
    /// Will be inserted a blob into this account.
    #[account(signer, mut)]
    pub blob_storage: Account<'info, BlobStorage>,

    /// PDA for storing the Merkle root.
    ///
    /// Initialize if the account is not existing.
    pub block_root: Account<'info, BlockRoot>,

    /// Soalan system program.
    pub system_program: Program<'info, System>,

    /// Solana sysvar to fetch the current slot number.
    pub clock: Sysvar<'info, Clock>,
}
