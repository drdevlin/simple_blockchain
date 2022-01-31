use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum BlockchainError {
    #[error("invalid chain length")]
    InvalidChainLength,
    #[error("invalid block")]
    InvalidBlock
}