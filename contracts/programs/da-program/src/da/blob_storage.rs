use std::collections::BTreeMap;

use anchor_lang::prelude::*;

/// Represents the account which stores blob data.
#[account]
#[derive(Default, Debug)]
pub struct BlobStorage {
    /// A map where keys are blob hashes and values are nested vectors of optional blob hashes.
    pub blobs: BTreeMap<[u8; 32], Vec<Vec<Option<[u8; 32]>>>>,
}

impl BlobStorage {
    /// Create a new blob storage with anempty blob BTreeMap.
    pub fn new() -> Self {
        BlobStorage {
            blobs: BTreeMap::new(),
        }
    }
}
