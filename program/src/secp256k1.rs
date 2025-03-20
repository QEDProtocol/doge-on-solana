/*
* NOTE: this is only here for testing on the solana testnet and should never be enabled on mainnet
* This is ONLY here to save some compute costs for testing since scrypt SP1 proofs take a lot of compute resources
*/

/*
* This is a dummy proof verifier which verifies an secp256k1 signature attestation:
* Valid signatures from the known public key are only created for payloads P which have the property
* Given x is 80 bytes,  y = sha256(x), and z = scrypt(x),
* P = sha256(y || z)
*/

#[cfg(target_os = "solana")]
use core::mem::MaybeUninit;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Secp256k1RecoverError {
    SignatureError,
    RecoveryError,
}

#[cfg(target_os = "solana")]
extern "C" {
    fn sol_secp256k1_recover(
        hash: *const [u8; 32],
        recovery_id: u64,
        signature: *const [u8; 64],
        result: *mut [u8; 64],
    ) -> u64;
}

#[inline(always)]
#[cfg(target_os = "solana")]
fn secp256k1_recover(
    hash: &[u8; 32],
    is_odd: bool,
    signature: &[u8; 64],
) -> Result<[u8; 64], Secp256k1RecoverError> {
    let mut out = MaybeUninit::<[u8; 64]>::uninit();
    unsafe {
        if sol_secp256k1_recover(
            hash.as_ptr() as *const [u8; 32],
            is_odd as u64,
            signature.as_ptr() as *const [u8; 64],
            out.as_mut_ptr() as *mut [u8; 64],
        ) == 0
        {
            Ok(out.assume_init())
        } else {
            Err(Secp256k1RecoverError::RecoveryError)
        }
    }
}
#[inline(always)]
#[cfg(not(target_os = "solana"))]
fn secp256k1_recover(
    hash: &[u8; 32],
    is_odd: bool,
    signature: &[u8; 64],
) -> Result<[u8; 64], Secp256k1RecoverError> {
    use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};

    // Parse the recoverable signature
    let signature: Signature = Signature::try_from(signature.as_ref())
        .map_err(|_| Secp256k1RecoverError::SignatureError)?;

    let rec_id = RecoveryId::from_byte(if is_odd { 1 } else { 0 }).unwrap();
    
    // Recover the public key
    let recovered: [u8; 64] = VerifyingKey::recover_from_prehash(hash, &signature, rec_id)
        .map_err(|_| Secp256k1RecoverError::RecoveryError)?
        .to_encoded_point(false)
        .as_bytes()[1..]
        .try_into()
        .map_err(|_| Secp256k1RecoverError::RecoveryError)?;

    Ok(recovered)
}

const FAKE_ZKP_SECP256K1_PUBLIC_KEY: [u8; 64] = [
    42, 226, 164, 253, 134, 160, 144, 70, 218, 238, 32, 82, 219, 9, 246, 74, 56, 240, 99, 112, 107,
    164, 4, 62, 203, 190, 172, 11, 92, 167, 93, 60, 166, 22, 174, 95, 242, 154, 59, 169, 14, 213,
    91, 29, 50, 196, 217, 111, 79, 90, 138, 60, 241, 216, 166, 63, 86, 65, 236, 179, 183, 174, 108,
    44,
];

pub fn verify_dummy_zkp(
    proof_data: &[u8],
    public_inputs: &[u8],
) -> Result<(), Secp256k1RecoverError> {
    if proof_data.len() != 260 {
        return Err(Secp256k1RecoverError::RecoveryError);
    } else if public_inputs.len() != 112 {
        return Err(Secp256k1RecoverError::RecoveryError);
    }
    let base_block_sha256 = solana_program::hash::hash(&public_inputs[0..80]).to_bytes();
    let mut combo: [u8; 64] = [0; 64];

    // sha256 hash
    combo[0..32].copy_from_slice(&base_block_sha256);

    // scrypt hash
    combo[32..64].copy_from_slice(&public_inputs[80..112]);

    let msg = solana_program::hash::hash(&combo).to_bytes();
    let is_odd = (proof_data[64]&1) == 1;
    let pubkey = secp256k1_recover(&msg, is_odd, &proof_data[0..64].try_into().unwrap())?;

    if pubkey.ne(&FAKE_ZKP_SECP256K1_PUBLIC_KEY) {
        Err(Secp256k1RecoverError::SignatureError)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::secp256k1::verify_dummy_zkp;

    #[test]
    fn verify_dummy_zkp_1() {
        let proof: [u8; 260] = [
            189, 56, 222, 153, 119, 206, 68, 140, 130, 213, 78, 193, 18, 194, 4, 158, 166, 142,
            183, 153, 40, 245, 248, 227, 183, 161, 85, 156, 69, 109, 111, 144, 120, 106, 94, 248,
            68, 234, 171, 205, 3, 197, 238, 8, 236, 170, 105, 183, 43, 165, 89, 64, 153, 90, 12,
            20, 29, 252, 21, 167, 245, 30, 105, 54, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        let public_inputs: [u8; 112] = [
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 82, 83, 6, 156, 20, 236,
            237, 249, 120, 116, 84, 134, 55, 94, 227, 116, 21, 233, 119, 245, 92, 219, 237, 172,
            49, 235, 238, 139, 243, 61, 209, 39,
        ];
        verify_dummy_zkp(&proof, &public_inputs).unwrap();
    }
}
