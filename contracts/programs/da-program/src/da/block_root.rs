use anchor_lang::prelude::*;

/// Represents the root account for block, typically storing a Merkle root.
#[account]
#[derive(Default, Debug)]
pub struct BlockRoot {
    /// The hash of all the merkle roots for each rollup during slot.
    pub hash: [u8; 32],
    /// The current slot number in Solana when this root is recorded.
    pub slot: u64,
}
