use anchor_lang::prelude::*;

pub type Hash = [u8; 32];

/// Represents an individual blob of data for rollup.
#[account]
#[derive(Default, Debug)]
pub struct Blob {
    /// Unique identifier for the rollup to which this blob belongs.
    ///
    /// We currently use Merkle root for simplicity.
    /// TODO: Migrate to hash of rollup.
    pub hash: Hash,
    /// Total number of blobs that make up the complete rollup.
    pub num_blobs: u64,
    /// Postion of this blob in the sequence of blob that form the entire rollup.
    pub position: u64,
    /// The actual size of the entire rollip.
    pub entire_size: u64,
    /// The actual data of this blob.
    pub body: Vec<u8>,
}
