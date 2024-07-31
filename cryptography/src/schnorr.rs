// cryptography/src/schnorr.rs

use k256::ecdsa::{signature::{Signer, Verifier}, Signature, SigningKey, VerifyingKey};
use rand_core::OsRng;

pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);
    (signing_key, verifying_key)
}

pub fn sign_message(message: &[u8], private_key: &SigningKey) -> Signature {
    private_key.sign(message)
}

pub fn verify_message(message: &[u8], signature: &Signature, public_key: &VerifyingKey) -> bool {
    public_key.verify(message, signature).is_ok()
}

