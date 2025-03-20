use borsh::BorshDeserialize;
use doge_light_client::core_data::{QDogeBlockHeader, QHash256};
use solana_program::program_error::ProgramError;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey, system_program,
};
use zerocopy::FromBytes;

use crate::assertions::{
    assert_pda, assert_program_owner, assert_same_pubkeys, assert_signer, assert_writable,
};
use crate::doge::constants::{QDogeNetworkConfig, QEDDogeChainState, Q_IBC_INNER_STATE_LEN};
use crate::instruction::accounts::{AppendBlockAccounts, CreateAccounts};
use crate::state::{Key, QEDDogeIBC};
use crate::utils::create_account;
use crate::zkp_verify;

pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data[0] == 0 {
        create(accounts, &instruction_data[1..])?;
        return Ok(());
    } else if instruction_data[0] == 1 && instruction_data.len() > 10 {
        let block_number = u32::from_le_bytes([
            instruction_data[1],
            instruction_data[2],
            instruction_data[3],
            instruction_data[4],
        ]);
        if instruction_data[5] == 1 && instruction_data.len() > 10 + 260 + 32 {
            // zkp mode

            let known_aux_pow_hash: [u8; 32] = instruction_data[6..(6 + 32)]
                .try_into()
                .map_err(|_| ProgramError::InvalidArgument)?;

            let proof_start_ind = instruction_data.len()-260;

            let header = QDogeBlockHeader::try_from_slice(&instruction_data[(6 + 32)..proof_start_ind])
            .map_err(|_| ProgramError::InvalidArgument)?;

            let proof_bytes = &instruction_data[proof_start_ind..(proof_start_ind + 260)];
            append_block_zkp(accounts, block_number, &header, known_aux_pow_hash, proof_bytes)?;

            return Ok(());
        } else if instruction_data[5] == 0{
            // no zkp for scrypt
            let header = QDogeBlockHeader::try_from_slice(&instruction_data[6..])
                .map_err(|_| ProgramError::InvalidArgument)?;
            append_block(accounts, block_number, &header, None)?;
            return Ok(());
        }else{
            return Err(ProgramError::InvalidArgument);
        }
    } else {
        return Err(ProgramError::InvalidArgument);
    }
}

fn create<'a>(accounts: &'a [AccountInfo<'a>], init_data_bytes: &[u8]) -> ProgramResult {
    // Accounts.
    let ctx = CreateAccounts::context(accounts)?;

    // Guards.
    let counter_bump = assert_pda(
        "qed_doge_ibc",
        ctx.accounts.qed_doge_ibc,
        &crate::ID,
        &QEDDogeIBC::seeds(ctx.accounts.authority.key),
    )?;
    assert_signer("authority", ctx.accounts.authority)?;
    assert_signer("payer", ctx.accounts.payer)?;
    assert_writable("payer", ctx.accounts.payer)?;
    assert_same_pubkeys(
        "system_program",
        ctx.accounts.system_program,
        &system_program::id(),
    )?;

    // Do nothing if the domain already exists.
    if !ctx.accounts.qed_doge_ibc.data_is_empty() {
        return Ok(());
    }

    // Create Counter PDA.
    let mut seeds = QEDDogeIBC::seeds(ctx.accounts.authority.key);
    let bump = [counter_bump];
    seeds.push(&bump);
    create_account(
        ctx.accounts.qed_doge_ibc,
        ctx.accounts.payer,
        ctx.accounts.system_program,
        QEDDogeIBC::LEN,
        &crate::ID,
        Some(&[&seeds]),
    )?;

    //msg!("init_data_bytes.len(): {}", init_data_bytes.len());

    QEDDogeChainState::ref_from_bytes(init_data_bytes)
        .map_err(|_| ProgramError::InvalidArgument)?;
    ctx.accounts.qed_doge_ibc.data.borrow_mut()[0] = Key::QEDDogeIBC as u8;
    ctx.accounts.qed_doge_ibc.data.borrow_mut()[1..33]
        .copy_from_slice(ctx.accounts.authority.key.as_ref());
    ctx.accounts.qed_doge_ibc.data.borrow_mut()[33..(33 + Q_IBC_INNER_STATE_LEN)]
        .copy_from_slice(init_data_bytes);

    Ok(())
}

fn append_block_zkp<'a>(
    accounts: &'a [AccountInfo<'a>],
    block_number: u32,
    block_header: &QDogeBlockHeader,
    known_scrypt_block_hash: QHash256,
    proof_bytes: &[u8],
) -> ProgramResult {
    let mut public_inputs = [0u8; 112];
    if block_header.header.is_aux_pow(){
        public_inputs[0..80].copy_from_slice(
            &((&block_header)
                .aux_pow
                .as_ref()
                .unwrap()
                .parent_block
                .to_bytes_fixed()),
        );
    }else{
        public_inputs[0..80].copy_from_slice(
            &((&block_header)
                .header
                .to_bytes_fixed()),
        );
    }
    public_inputs[80..112].copy_from_slice(&known_scrypt_block_hash);

    // Get the SP1 Groth16 verification key from the `sp1-solana` crate.
    zkp_verify::verify_zkp(&proof_bytes, &public_inputs)?;

    append_block(accounts, block_number, block_header, Some(known_scrypt_block_hash))
}

fn append_block<'a>(
    accounts: &'a [AccountInfo<'a>],
    block_number: u32,
    block_header: &QDogeBlockHeader,
    known_scrypt_block_hash: Option<QHash256>,
) -> ProgramResult {
    // Accounts.
    let ctx = AppendBlockAccounts::context(accounts)?;

    // Guards.
    assert_signer("authority", ctx.accounts.authority)?;
    assert_pda(
        "qed_doge_ibc",
        ctx.accounts.qed_doge_ibc,
        &crate::ID,
        &QEDDogeIBC::seeds(ctx.accounts.authority.key),
    )?;
    assert_program_owner("qed_doge_ibc", ctx.accounts.qed_doge_ibc, &crate::ID)?;

    let auth: [u8; 32] = ctx.accounts.qed_doge_ibc.data.borrow()[1..33]
        .try_into()
        .map_err(|_| ProgramError::InvalidArgument)?;
    assert_same_pubkeys("authority", ctx.accounts.authority, &auth.into())?;

    let mut p = ctx.accounts.qed_doge_ibc.data.borrow_mut();
    let doge_chain_state =
        QEDDogeChainState::mut_from_bytes(&mut p[33..(33 + Q_IBC_INNER_STATE_LEN)])
            .map_err(|_| ProgramError::InvalidArgument)?;

    // append block
    doge_chain_state.append_block::<QDogeNetworkConfig>(block_number, &block_header, known_scrypt_block_hash)?;
    //doge_state.save(ctx.accounts.qed_doge_ibc)

    Ok(())
}
