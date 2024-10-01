use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Custom error to throw")]
    CustomError,
}
