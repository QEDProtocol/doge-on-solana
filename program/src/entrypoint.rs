use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
    program_error::PrintProgramError, pubkey::Pubkey,
};

use crate::{alt_entrypoint, error::QEDDogeIBCError, processor};

alt_entrypoint!(process_instruction);
fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = processor::process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<QEDDogeIBCError>();
        return Err(error);
    }
    Ok(())
}
