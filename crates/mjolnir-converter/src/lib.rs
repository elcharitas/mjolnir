use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractType {
    Ink,
    Solidity,
}

impl Display for ContractType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractType::Ink => write!(f, "Ink"),
            ContractType::Solidity => write!(f, "Solidity"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionConfig {
    pub target: ContractType,
    pub optimize: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionResult {
    pub converted_code: String,
    pub target_type: ContractType,
    pub compilation_output: Option<String>,
}

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Failed to parse contract: {0}")]
    ParseError(String),
    #[error("Unsupported conversion from {from} to {to}")]
    UnsupportedConversion {
        from: ContractType,
        to: ContractType,
    },
    #[error("Compilation error: {0}")]
    CompilationError(String),
}

pub trait ContractConverter {
    fn convert(
        &self,
        code: &str,
        config: &ConversionConfig,
    ) -> Result<ConversionResult, ConversionError>;
    fn detect_type(&self, code: &str) -> Option<ContractType>;
}
