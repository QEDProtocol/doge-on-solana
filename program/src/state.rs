use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::doge::constants::Q_IBC_INNER_STATE_LEN;
use crate::error::QEDDogeIBCError;

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub enum Key {
    Uninitialized,
    QEDDogeIBC,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct QEDDogeIBC {
    pub key: Key,
    pub authority: Pubkey,
    pub value: u32,
}

impl QEDDogeIBC {
    pub const LEN: usize = 1 + 32 + Q_IBC_INNER_STATE_LEN;

    pub fn seeds(authority: &Pubkey) -> Vec<&[u8]> {
        vec!["qed_doge_ibc".as_bytes(), authority.as_ref()]
    }

    pub fn find_pda(authority: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&Self::seeds(authority), &crate::ID)
    }

    pub fn load(_account: &AccountInfo) -> Result<Self, ProgramError> {
        Err(QEDDogeIBCError::DeserializationError.into())
        /* 
        let mut bytes: &[u8] = &(*account.data).borrow();
        QEDDogeIBC::deserialize(&mut bytes).map_err(|error| {
            msg!("Error: {}", error);
            QEDDogeIBCError::DeserializationError.into()
        })*/
    }

    pub fn save(&self, _account: &AccountInfo) -> ProgramResult {
        Err(QEDDogeIBCError::SerializationError.into())
        /*
        borsh::to_writer(&mut account.data.borrow_mut()[..], self).map_err(|error| {
            msg!("Error: {}", error);
            QEDDogeIBCError::SerializationError.into()
        })
        */
    }
}
