use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;


/// Deserialize the input arguments.
///
/// Behaves like [`solana_program::entrypoint::deserialize`] except for special
/// handling of empty instruction data.
///
/// If the instruction data is empty, the instruction data is read from the last
/// account passed to the instruction.  The data of the account is interpreted
/// as length-prefixed sequence of bytes with length being an unsigned 32-bit
/// integer using little endian encoding.  The account used to read the account
/// data is not returned with the rest of the accounts.
///
/// # Safety
///
/// Must be called with pointer to properly serialised instruction such as done
/// by the Solana runtime.  See [`solana_program::entrypoint::deserialize`].
pub unsafe fn deserialize<'a>(
    input: *mut u8,
) -> Result<(&'a Pubkey, Vec<AccountInfo<'a>>, &'a [u8]), ProgramError> {
    // SAFETY: Caller promises this is safe.
    let (program_id, mut accounts, mut instruction_data) =
        unsafe { solana_program::entrypoint::deserialize(input) };

    // If instruction data is empty, the actual instruction data comes from the
    // last account passed in the call.
    if instruction_data.is_empty() && accounts.len() > 1 {
        //msg!("Instruction data is empty, reading from last account");
        instruction_data = get_ix_data(&mut accounts)?;
    }

    Ok((program_id, accounts, instruction_data))
}


/// Interprets data in the last account as instruction data.
fn get_ix_data<'a>(
    accounts: &mut Vec<AccountInfo<'a>>,
) -> Result<&'a [u8], ProgramError> {
    //msg!("in ix");
    let account = accounts.pop().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let data = std::rc::Rc::try_unwrap(account.data);
    let data = data.ok().unwrap().into_inner();
    const BASE_LEN: usize = 37;

    if data.len() < BASE_LEN+4 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let len = u32::from_le_bytes(data[BASE_LEN..(BASE_LEN+4)].try_into().unwrap());

    let len: usize =
        usize::try_from(len).map_err(|_| ProgramError::ArithmeticOverflow)?;



    Ok(&data[(BASE_LEN+4)..(BASE_LEN+4+len)])
}


/// Declare the program entrypoint and set up global handlers.
///
/// Analogous to [`solana_program::entrypoint`] macro with additional handling
/// of empty instruction data as described in [`deserialize`].
#[macro_export]
macro_rules! alt_entrypoint {
    ($process_instruction:ident) => {
        /// Solana program entry point.
        ///
        /// # Safety
        ///
        /// Must be called with pointer to properly serialised instruction such
        /// as done by the Solana runtime.
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            // SAFETY: Caller promises this is safe.
            let parsed = unsafe { $crate::alt_entrypoint::deserialize(input) };
            let (program_id, accounts, data) = match parsed {
                Ok(it) => it,
                Err(error) => return error.into(),
            };
            match $process_instruction(program_id, &accounts, data) {
                Ok(()) => $crate::alt_entrypoint::__private::SUCCESS,
                Err(error) => error.into(),
            }
        }
        $crate::alt_entrypoint::__private::solana_program::custom_heap_default!();
        $crate::alt_entrypoint::__private::solana_program::custom_panic_default!();
    };
}


#[doc(hidden)]
pub mod __private {
    pub use solana_program;
    pub use solana_program::entrypoint::SUCCESS;
}