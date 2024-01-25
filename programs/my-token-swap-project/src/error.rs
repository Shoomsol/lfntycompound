// error.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum TokenSwapError {
    #[msg("Insufficient Funds for the transaction.")]
    InsufficientFunds,

    #[msg("Token transfer failed.")]
    TransferFailed,

    #[msg("Token minting failed.")]
    MintFailed,

    #[msg("Token burning failed.")]
    BurnFailed,

    // Add other custom errors as needed
}
