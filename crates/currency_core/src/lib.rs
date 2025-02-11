use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod providers;

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct CurrencyCode(String);

#[derive(Debug, Error)]
pub enum CurrencyCodeParsingError {
    #[error("The code provided must be 3 characters")]
    InvalidLength,
}

impl CurrencyCode {
    pub fn new(code: impl Into<String>) -> Self {
        let code: String = code.into();

        if code.len() != 3 {
            panic!(
                "Attempted construction of currency code with invalid code {}",
                code
            );
        }

        Self(code)
    }
}
