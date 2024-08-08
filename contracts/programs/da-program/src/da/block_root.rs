use anchor_lang::{prelude::*, solana_program::blake3::hashv};

use crate::common::Hash;

/// Represents the root account for block, typically storing a Merkle root.
#[account]
#[derive(Default, Debug)]
pub struct BlockRoot {
    /// The hash of all the merkle roots for each rollup during slot.
    pub hash: Hash,
    /// The current slot number in Solana when this root is recorded.
    pub slot: u64,
}

impl BlockRoot {
    /// Create a new instance with default values.
    pub fn new() -> Self {
        BlockRoot {
            hash: [0u8; 32],
            slot: 0,
        }
    }

    /// Updates the root of the BlockRoot with given block root and slot number.
    /// 
    /// If the slot number is greater than the current slot, the hash is updated.
    /// Otherwise, `block_root_accumulator` is used to merge the current digest with the given block root.
    pub fn update_root(&mut self, block_root: &Hash, slot: u64) {
        if slot > self.slot {
            self.hash = *block_root;

            // TODO:
        } else {
            self.hash = block_root_accumulator(&self.hash, block_root);
        }
    }
}

fn block_root_accumulator(current_root: &Hash, block_root: &Hash) -> Hash {
    hashv(&[current_root.as_ref(), block_root.as_ref()]).0
}
