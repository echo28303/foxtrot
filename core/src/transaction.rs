use k256::ecdsa::{Signature as EcdsaSignature, SigningKey, VerifyingKey};
use k256::ecdsa::signature::{Signer, Verifier};
use serde::{Serialize, Deserialize};
use serde::de::{self, Deserializer, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use zero_knowledge::proof::{ZkProver, Proof, create_trace, verify_proof};
use winterfell::math::fields::f128::BaseElement;
use crate::block::Signature;
use std::fmt;

#[derive(Clone, Debug)]
pub struct ProofWrapper(pub Proof);

impl Serialize for ProofWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let proof_bytes = self.0.to_bytes();
        serializer.serialize_bytes(&proof_bytes)
    }
}

impl<'de> Deserialize<'de> for ProofWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProofVisitor;

        impl<'de> Visitor<'de> for ProofVisitor {
            type Value = ProofWrapper;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a byte array representing a zero-knowledge proof")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Proof::from_bytes(v).map(ProofWrapper).map_err(E::custom)
            }
        }

        deserializer.deserialize_bytes(ProofVisitor)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: Signature,
    pub zk_proof: ProofWrapper, // Use the wrapper type
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64, private_key: &SigningKey) -> Self {
        let message = format!("{}:{}:{}", sender, receiver, amount);
        let signature = private_key.sign(message.as_bytes());

        // Generate zero-knowledge proof for the transaction
        let prover = ZkProver::new(create_trace(vec![
            BaseElement::new(amount.into()), // Using amount as an example input
        ]));
        let zk_proof = prover.generate_proof().expect("Failed to generate proof");

        Self {
            sender,
            receiver,
            amount,
            signature: Signature(signature),
            zk_proof: ProofWrapper(zk_proof), // Use the wrapper type
        }
    }

    pub fn verify(&self, public_key: &VerifyingKey) -> bool {
        let message = format!("{}:{}:{}", self.sender, self.receiver, self.amount);
        if public_key.verify(message.as_bytes(), &self.signature.0).is_err() {
            return false;
        }

        // Verify the zero-knowledge proof
        verify_proof(self.zk_proof.0.clone()).is_ok()
    }
}
