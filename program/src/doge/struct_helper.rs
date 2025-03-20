
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "borsh")]
use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankType;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes};


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "borsh", derive(BorshSerialize, BorshDeserialize))]
#[derive(ShankType, Default, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct QSQDogeBlockHeader {
    pub header: QSQStandardBlockHeader,
    pub aux_pow: Option<QSQAuxPow>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "borsh", derive(BorshSerialize, BorshDeserialize))]
#[derive(ShankType, Default, Clone, Debug, PartialEq)]
pub struct QSQDogeBlock {
    pub header: QSQStandardBlockHeader,
    pub aux_pow: Option<QSQAuxPow>,
    pub transactions: Vec<QSBTCTransaction>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "borsh", derive(BorshSerialize, BorshDeserialize))]
#[derive(ShankType, Default, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct QSQMerkleBranch {
    pub hashes: Vec<[u8; 32]>,
    pub side_mask: u32,
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "borsh", derive(BorshSerialize, BorshDeserialize))]
#[derive(ShankType, Default, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct QSQAuxPow {
    pub coinbase_transaction: QSBTCTransaction,
    pub block_hash: [u8; 32],
    pub coinbase_branch: QSQMerkleBranch,
    pub blockchain_branch: QSQMerkleBranch,
    pub parent_block: QSQStandardBlockHeader,
}



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "borsh", derive(BorshSerialize, BorshDeserialize))]
#[derive(ShankType, Default, Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, FromBytes, IntoBytes, Immutable)]
pub struct QSQStandardBlockHeader {
    pub version: u32,
    pub previous_block_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u32,
    pub bits: u32,
    pub nonce: u32,
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "borsh", derive(BorshSerialize, BorshDeserialize))]
#[derive(ShankType, Default, PartialEq, Clone, Debug, Eq, Ord, PartialOrd)]
pub struct QSBTCTransaction {
    pub version: u32,
    pub inputs: Vec<QSBTCTransactionInput>,
    pub outputs: Vec<QSBTCTransactionOutput>,
    pub locktime: u32,
}




#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "borsh", derive(BorshSerialize, BorshDeserialize))]
#[derive(ShankType, Default, PartialEq, Clone, Debug, Eq, Ord, PartialOrd)]
pub struct QSBTCTransactionOutput {
    pub value: u64,
    pub script: Vec<u8>,
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "borsh", derive(BorshSerialize, BorshDeserialize))]
#[derive(ShankType, Default, PartialEq, Clone, Debug, Eq, Ord, PartialOrd)]
pub struct QSBTCTransactionInput {
    pub hash: [u8; 32],
    pub index: u32,
    pub script: Vec<u8>,
    pub sequence: u32,
    // TODO: implement witnesses for advanced transactions supported by bitcoin
    // pub witness: Vec<u8>,
}