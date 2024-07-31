use k256::ecdsa::{Signature as EcdsaSignature, SigningKey, VerifyingKey};
use k256::ecdsa::signature::{Signer, Verifier};
use serde::{Serialize, Deserialize};
use serde::de::{self, Deserializer, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use std::fmt;
use crate::transaction::Transaction;

#[derive(Clone, Debug)]
pub struct Signature(pub EcdsaSignature);

impl Signature {
    pub fn from_slice(bytes: &[u8]) -> Self {
        // Assuming EcdsaSignature implements From<&[u8]>
        Signature(EcdsaSignature::from_der(bytes).expect("Failed to create signature"))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u128,
    pub nonce: u64,
    pub miner: String,
    pub signature: Signature,
    pub reward: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        index: u64,
        previous_hash: String,
        timestamp: u128,
        nonce: u64,
        miner: String,
        signature: Signature,
        reward: u64,
        transactions: Vec<Transaction>,
    ) -> Self {
        Self {
            index,
            previous_hash,
            timestamp,
            nonce,
            miner,
            signature,
            reward,
            transactions,
        }
    }

    pub fn calculate_hash(
        index: u64,
        previous_hash: &str,
        timestamp: u128,
        nonce: u64,
        miner: &str,
        signature: &Signature,
        reward: u64,
    ) -> String {
        let data = format!(
            "{}{}{}{}{}{:?}{}",
            index, previous_hash, timestamp, nonce, miner, signature.0, reward
        );
        blake3::hash(data.as_bytes()).to_hex().to_string()
    }

    pub fn sign(&mut self, private_key: &SigningKey) {
        let message = Self::calculate_hash(
            self.index,
            &self.previous_hash,
            self.timestamp,
            self.nonce,
            &self.miner,
            &self.signature,
            self.reward,
        );
        let sig = private_key.sign(message.as_bytes());
        self.signature = Signature(sig);
    }

    pub fn verify_signature(&self, public_key: &VerifyingKey) -> bool {
        let message = Self::calculate_hash(
            self.index,
            &self.previous_hash,
            self.timestamp,
            self.nonce,
            &self.miner,
            &self.signature,
            self.reward,
        );
        public_key.verify(message.as_bytes(), &self.signature.0).is_ok()
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let binding = self.0.to_bytes();
        let sig_bytes: &[u8] = binding.as_ref();
        serializer.serialize_bytes(sig_bytes)
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SigVisitor;

        impl<'de> Visitor<'de> for SigVisitor {
            type Value = Signature;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a byte array representing a signature")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                EcdsaSignature::from_der(v).map(Signature).map_err(E::custom)
            }
        }

        deserializer.deserialize_bytes(SigVisitor)
    }
}
