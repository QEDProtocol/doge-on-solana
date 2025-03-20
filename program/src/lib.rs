pub mod doge;
pub mod alt_entrypoint;
pub mod assertions;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;
pub mod zkp_verify;

#[cfg(feature = "dummy_zkp")]
pub mod secp256k1;

pub use solana_program;

solana_program::declare_id!("Fu4pdQiKyrBKnUyvtbGUPkcg2HKp9d6Ji8JLGvS6E7UQ");
