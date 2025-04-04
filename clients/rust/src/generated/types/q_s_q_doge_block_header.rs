//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::generated::types::QSQAuxPow;
use crate::generated::types::QSQStandardBlockHeader;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QSQDogeBlockHeader {
    pub header: QSQStandardBlockHeader,
    pub aux_pow: Option<QSQAuxPow>,
}
