use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

use crate::doge::struct_helper::QSQDogeBlockHeader;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum QEDDogeIBCInstruction {
    /// Creates the qed_doge_ibc account derived from the provided authority.
    #[account(0, writable, name="qed_doge_ibc", desc = "The program derived address of the qed_doge_ibc account to create (seeds: ['qed_doge_ibc', authority])")]
    #[account(1, signer, name="authority", desc = "The authority of the qed_doge_ibc")]
    #[account(2, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(3, name="system_program", desc = "The system program")]
    Create { init_data: Vec<u8>},

    /// Appends a block to the state
    #[account(0, writable, name="qed_doge_ibc", desc = "The program derived address of the qed_doge_ibc account to increment (seeds: ['qed_doge_ibc', authority])")]
    #[account(1, signer, name="authority", desc = "The authority of the qed_doge_ibc")]
    AppendBlock { 
        block_number: u32,
        pow_hash_mode: u8,
        block_header: QSQDogeBlockHeader,
    },

    #[account(0, writable, name="qed_doge_ibc", desc = "The program derived address of the qed_doge_ibc account to increment (seeds: ['qed_doge_ibc', authority])")]
    #[account(1, signer, name="authority", desc = "The authority of the qed_doge_ibc")]
    AppendBlockZKP { 
        block_number: u32,
        pow_hash_mode: u8,
        known_pow_hash: [u8; 32],
        block_header: QSQDogeBlockHeader,
        scrypt_proof: [u8; 260],
    },
}
