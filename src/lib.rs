//! # Simple Blockchain
//! 
//! A simple blockchain inspired by [Mario Zupan](https://blog.logrocket.com/how-to-build-a-blockchain-in-rust/).
pub use self::block::Block;
pub use self::blockchain::Blockchain;

pub mod block;
pub mod blockchain;
pub mod error;
mod helpers;