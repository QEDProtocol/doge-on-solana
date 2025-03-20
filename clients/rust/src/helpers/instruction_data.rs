use crate::doge_chain_state::QEDDogeChainState;
use doge_light_client::core_data::{QDogeBlockHeader, QHash256};
use zerocopy::IntoBytes;

pub fn gen_instruction_data_create(init_state: &QEDDogeChainState) -> Vec<u8> {
    let state_data = init_state.as_bytes().to_vec();
    println!("state_data_len: {}", state_data.len());
    let full_len = state_data.len() + 1 + 4;
    let inner_length = (state_data.len() + 1) as u32;
    let mut instruction_data = Vec::with_capacity(full_len);
    instruction_data.extend_from_slice(&inner_length.to_le_bytes()); // buffer len
    instruction_data.push(0u8); // instruction id
    instruction_data.extend_from_slice(&state_data); // state data
    instruction_data
}
pub fn gen_instruction_data_append_block(
    block_number: u32,
    block_header: &QDogeBlockHeader,
) -> Vec<u8> {
    let block_header_bytes = borsh::to_vec(&block_header).unwrap();
    let full_len = block_header_bytes.len() + 1 + 4 + 4 + 1;
    let inner_length = (block_header_bytes.len() + 1 + 4 + 1) as u32;
    let mut instruction_data = Vec::with_capacity(full_len);
    instruction_data.extend_from_slice(&inner_length.to_le_bytes()); // buffer len
    instruction_data.push(1u8); // instruction id
    instruction_data.extend(&block_number.to_le_bytes());
    instruction_data.push(0u8); // non-zkp mode
    instruction_data.extend_from_slice(&block_header_bytes); // state data
    instruction_data
}
pub fn gen_instruction_data_append_block_zkp(
    block_number: u32,
    block_header: &QDogeBlockHeader,
    proof: &[u8],
    scrypt_hash: QHash256,
) -> Vec<u8> {
    let block_header_bytes = borsh::to_vec(&block_header).unwrap();
    let full_len = block_header_bytes.len() + 1 + 4 + 4 + 1 + 32 + 260;
    let inner_length = (block_header_bytes.len() + 1 + 4 + 1 + 32 + 260) as u32;
    let mut instruction_data = Vec::with_capacity(full_len);
    instruction_data.extend_from_slice(&inner_length.to_le_bytes()); // buffer len
    instruction_data.push(1u8); // instruction id
    instruction_data.extend(&block_number.to_le_bytes());
    instruction_data.push(1u8); // zkp mode
    instruction_data.extend_from_slice(&scrypt_hash); // state data
    instruction_data.extend_from_slice(&block_header_bytes); // state data
    instruction_data.extend_from_slice(&proof); // state data
    instruction_data
}
pub fn gen_instruction_data_append_block_zkp_start(
    block_number: u32,
    block_header: &QDogeBlockHeader,
    scrypt_hash: QHash256,
) -> Vec<u8> {
    let block_header_bytes = borsh::to_vec(&block_header).unwrap();
    let full_len = block_header_bytes.len() + 1 + 4 + 4 + 1 + 32 + 260;
    let inner_length = (block_header_bytes.len() + 1 + 4 + 1 + 32 + 260) as u32;
    let mut instruction_data = Vec::with_capacity(full_len);
    instruction_data.extend_from_slice(&inner_length.to_le_bytes()); // buffer len
    instruction_data.push(1u8); // instruction id
    instruction_data.extend(&block_number.to_le_bytes());
    instruction_data.push(1u8); // zkp mode
    instruction_data.extend_from_slice(&scrypt_hash); // state data
    instruction_data.extend_from_slice(&block_header_bytes); // state data
    instruction_data
}
