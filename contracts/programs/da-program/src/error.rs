use anchor_lang::prelude::*;

/// Custom Errors
#[error_code]
pub enum ErrorCode {
    #[msg(format!("Max blob size is {}", BLOB_SIZE))]
    BlobSizeTooLong,
}
