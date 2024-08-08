use std::collections::BTreeMap;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::blake3::hashv;

use crate::common::*;

/// Represents the account which stores blob data.
#[account]
#[derive(Default, Debug)]
pub struct BlobStorage {
    /// A map where keys are blob hashes and values are nested vectors of optional blob hashes.
    pub blobs: BTreeMap<Hash, Vec<Vec<Option<Hash>>>>,
}

impl BlobStorage {
    /// Create a new blob storage with anempty blob BTreeMap.
    pub fn new() -> Self {
        BlobStorage {
            blobs: BTreeMap::new(),
        }
    }

    /// Remove the blob from BTreeMap according to given hash.
    pub fn remove(&mut self, hash: &Hash) {
        self.blobs.remove(hash);
    }

    /// Clear the BTreeMap
    pub fn clear(&mut self) {
        self.blobs = BTreeMap::new();
    }

    /// Update the Merkle Tree by inserting blob hash into BTreeMap.
    pub fn update_merkle_tree(&mut self, blob: Blob) {
        let Blob {
            hash,
            num_blobs,
            position,
            entire_size,
            body,
        } = blob;

        // Initialize the levels of the Merkle tree if it is not existing for the given blob's hash
        let merkle_tree = self.blobs.entry(hash).or_insert_with(|| {
            let mut vec = Vec::new();
            let mut num = num_blobs as usize;

            while num > 1 {
                vec.push(vec![None; num]);
                num = (num + 1) / 2;
            }

            vec.push(vec![None]);

            vec
        });

        // Add the entire size(little-endian bytes) of rollup in the front of blobs as discriminator
        let blob_hash = {
            let mut extended = Vec::with_capacity(8 + body.len());

            extended.extend_from_slice(&entire_size.to_le_bytes());
            extended.extend(body);

            hashv(&[&extended]).to_bytes()
        };

        merkle_tree[0][position as usize] = Some(blob_hash);

        // Loop move up the Merkle tree levels, checking nodes two by two
        let mut current_level = 0;
        let mut current_index = position as usize;

        while current_level < merkle_tree.len() - 1 {
            // 
            if current_index % 2 == 1
                && merkle_tree[current_level][current_index].is_some()
                && merkle_tree[current_level][current_index - 1].is_some()
            {
                let left = merkle_tree[current_level][current_index - 1].as_ref().unwrap();
                let right = merkle_tree[current_level][current_index].as_ref().unwrap();

                let merged = hashv(&[left, right]).to_bytes();

                merkle_tree[current_level + 1][current_index / 2] = Some(merged);

                current_level += 1;
                current_index /= 2;
            } else if position == num_blobs - 1
                && current_index % 2 ==0
                && merkle_tree[current_level][current_index].is_some()
            {
                // Unpaired node which is the last one propagate up
                merkle_tree[current_level + 1][current_index / 2] = merkle_tree[current_level][current_index].clone();

                current_level += 1;
                current_index /= 2;
            } else {
                break;
            }
        }
    }

    /// Fetch the Merkle tree for a specific hash
    pub fn get_merkle_root(&self, hash: &Hash) -> Option<Hash> {
        self.blobs
            .get(hash)
            .and_then(|levels| levels.last()?.first().cloned())
            .flatten()
    }

    /// Check if the Merkle tree is complete according to a given hash
    pub fn is_complete(&self, hash: &Hash) -> bool {
        self.get_merkle_root(hash).is_some()
    }
}
