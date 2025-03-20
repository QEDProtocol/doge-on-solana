use qed_data_loader_v3_program_client::instructions::{
    CloseBuilder, InitializeBufferBuilder, WriteBuilder,
};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

/// Creates an
/// [InitializeBuffer](enum.QLoaderV3Instruction.html)
/// instruction.
pub fn buffer_initialize_buffer_instruction(
    source_address: &Pubkey,
    authority_address: &Pubkey,
) -> Instruction {
    InitializeBufferBuilder::new()
        .buffer_authority(*authority_address)
        .source_account(*source_address)
        .instruction()
}

/// Creates a
/// [Write](enum.QEDDataLoaderInstruction.html)
/// instruction.
pub fn buffer_write_instruction(
    buffer_address: &Pubkey,
    authority_address: &Pubkey,
    offset: u32,
    bytes: Vec<u8>,
) -> Instruction {
    println!("buffer_write_instruction, buffer_address: {}, authority_address: {}, offset: {}, bytes.len(): {}", buffer_address, authority_address, offset, bytes.len());
    WriteBuilder::new()
        .buffer_account(*buffer_address)
        .buffer_authority(*authority_address)
        .offset(offset)
        .bytes(bytes)
        .instruction()
}

/// Creates a
/// [Close](enum.QEDDataLoaderInstruction.html)
/// instruction.
pub fn buffer_close_instruction(
    buffer_or_program_data_address: &Pubkey,
    destination_address: &Pubkey,
    authority_address: Option<Pubkey>,
    program_address: Option<Pubkey>,
) -> Instruction {
    CloseBuilder::new()
        .destination_account(*destination_address)
        .buffer_or_program_data_account(*buffer_or_program_data_address)
        .authority(authority_address)
        .program_account(program_address)
        .instruction()
}
