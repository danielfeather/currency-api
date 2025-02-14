use std::{collections::HashMap, fmt, str::FromStr};

use chrono::{DateTime, Local, Utc};
use providers::open_exchange_rates::Rates;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod providers;

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct CurrencyCode(pub String);

#[derive(Debug, Error)]
pub enum CurrencyCodeParsingError {
    #[error("The code provided must be 3 characters")]
    InvalidLength,
}

impl CurrencyCode {
    pub fn new(code: &str) -> Self {
        if code.len() != 3 {
            panic!(
                "Attempted construction of currency code with invalid code {}",
                code
            );
        }

        Self(code.into())
    }
}

impl FromStr for CurrencyCode {
    type Err = CurrencyCodeParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err(CurrencyCodeParsingError::InvalidLength);
        }

        Ok(Self(s.into()))
    }
}

// impl fmt::Display for CurrencyCode {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self)
//     }
// }

#[derive(Debug)]
pub struct Currency {
    pub code: CurrencyCode,
    pub amount: f32,
}

impl Currency {
    pub fn new(code: CurrencyCode, amount: f32) -> Self {
        Currency { code, amount }
    }
}

#[derive(Debug, Error)]
pub enum CurrencyParsingError {
    #[error("Currency code provided is invalid")]
    InvalidCurrencyCode,
    #[error("Amount of currency provided is invalid")]
    InvalidAmount,
    #[error("Currency was not provided in a known format")]
    IncorrectFormat,
}

impl FromStr for Currency {
    type Err = CurrencyParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(" ").collect();

        if parts.len() != 2 {
            return Err(CurrencyParsingError::IncorrectFormat);
        }

        let code = CurrencyCode::from_str(parts[1])
            .map_err(|_| CurrencyParsingError::InvalidCurrencyCode)?;

        let amount = f32::from_str(parts[0]).map_err(|_| CurrencyParsingError::InvalidAmount)?;

        Ok(Currency { code, amount })
    }
}

#[derive(Debug, Default)]
pub struct Exchange {
    from: Option<Currency>,
    to: Option<CurrencyCode>,
}

impl Exchange {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to(&mut self, code: CurrencyCode) -> &mut Self {
        self.to = Some(code);
        self
    }

    pub fn from(&mut self, currency: Currency) -> &mut Self {
        self.from = Some(currency);
        self
    }

    pub fn exchange(self, rates: &Rates) -> Currency {
        let from = self.from.expect("No from currency provided");
        let to = self.to.expect("No conversion currency specified");

        if to == from.code {
            return from;
        }

        let Some(base_rate) = rates.rates.get(&from.code) else {
            panic!("Unable to convert to base rate");
        };

        let Some(to_base_rate) = rates.rates.get(&to) else {
            panic!("Unable to find ro exchange rate");
        };

        let converted_to_base = from.amount * base_rate;

        let converted = converted_to_base * to_base_rate;

        Currency {
            code: to,
            amount: converted,
        }
    }
}

#[test]
fn test_change() {
    let mut rates = HashMap::new();

    rates.insert(CurrencyCode("AZN".to_string()), 1.7);
    rates.insert(CurrencyCode("GBP".to_string()), 0.794593);
    rates.insert(CurrencyCode("USD".to_string()), 1.0);

    let exchange_info = Rates {
        disclaimer: "Usage subject to terms: https://openexchangerates.org/terms".to_string(),
        license: "https://openexchangerates.org/license".to_string(),
        timestamp: Local::now().to_utc(),
        base: CurrencyCode("USD".to_string()),
        rates,
    };

    let mut currency_builder = Exchange::new();

    currency_builder
        .from(Currency::new(CurrencyCode::new("GBP"), 1.0))
        .to(CurrencyCode::new("USD"));

    let currency = currency_builder.exchange(&exchange_info);

    assert_eq!(currency.amount, 0.794593)
}
