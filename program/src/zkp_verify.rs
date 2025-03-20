use solana_program::{entrypoint::ProgramResult, program_error::ProgramError};

#[cfg(not(feature = "dummy_zkp"))]
const SCRYPT_VERIFIER_VKEY_HASH: &str =
    "0x00445074d00755f11ef24bd47b123b5f2008d68cdb8423a497912dc7a159774d";



/*
 Because scrypt_1024_1_1_256 takes a lot of compute power, 
 we instead generate a zero knowledge proof which proves scrypt_1024_1_1_256(public_inputs[0..80]) == public_inputs[80..112] 
 and then verify the zkProof on chain.
*/
pub fn verify_zkp(proof_data: &[u8], public_inputs: &[u8]) -> ProgramResult {
    if proof_data.len() != 260 {
        return Err(ProgramError::InvalidInstructionData);
    }else if public_inputs.len() != 112 {
        return Err(ProgramError::InvalidInstructionData);
    }
    
    // if we are on testnet, save some compute time by verifying a secp256k1 signature attesting to the scrypt instead of the SP1 proof
    #[cfg(feature = "dummy_zkp")]
    crate::secp256k1::verify_dummy_zkp(proof_data, public_inputs).map_err(|_| ProgramError::InvalidInstructionData)?;

    // verify an SP1 zkProof that proves the scrypt_1024_1_1_256(<pow_block_header>) matches the last 32 bytes of the public inputs
    #[cfg(not(feature = "dummy_zkp"))]
    sp1_solana::verify_proof(
        &proof_data,
        &public_inputs,
        &SCRYPT_VERIFIER_VKEY_HASH,
        &sp1_solana::GROTH16_VK_4_0_0_RC3_BYTES,
    ).map_err(|_| ProgramError::InvalidInstructionData)?;

    Ok(())
}