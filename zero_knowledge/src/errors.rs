use winter_prover::ProverError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    ProverError(ProverError),
    CustomMessage(String),  // New variant to handle custom string messages
    Other(Box<dyn Error>),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::ProverError(e) => write!(f, "Prover error: {}", e),
            CustomError::CustomMessage(msg) => write!(f, "Error: {}", msg),
            CustomError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl From<ProverError> for CustomError {
    fn from(error: ProverError) -> Self {
        CustomError::ProverError(error)
    }
}

impl From<Box<dyn Error>> for CustomError {
    fn from(error: Box<dyn Error>) -> Self {
        CustomError::Other(error)
    }
}
