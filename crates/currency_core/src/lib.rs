use std::{collections::HashMap, fmt, str::FromStr};

use chrono::Local;
use providers::open_exchange_rates::Rates;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod providers;

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
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

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Error)]
pub enum ExchangeError {
    #[error("The exchange was configured incorrect, missing 'to' or 'from' currency")]
    InvalidConfiguration,
    #[error("Currency code provided was invalid")]
    CurrencyError(CurrencyParsingError),
    #[error("No exchange rate available for {0}")]
    UnsupportedCurrency(CurrencyCode),
}

/// Builder for converting currencies
#[derive(Debug, Default)]
pub struct Exchange {
    from: Option<Currency>,
    to: Option<CurrencyCode>,
}

impl Exchange {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the currency to convert to
    pub fn to(&mut self, code: CurrencyCode) -> &mut Self {
        self.to = Some(code);
        self
    }

    /// Set the currency to convert from
    pub fn from(&mut self, currency: Currency) -> &mut Self {
        self.from = Some(currency);
        self
    }

    /// Convert the currency to the desired currency using cross-currency triangulation,
    /// if no direct conversion is available
    pub fn exchange(self, rates: &Rates) -> Result<Currency, ExchangeError> {
        let from = self.from.expect("No from currency provided");
        let to = self.to.expect("No conversion currency specified");

        let Some(base_rate) = rates.rates.get(&from.code) else {
            return Err(ExchangeError::UnsupportedCurrency(from.code));
        };

        let Some(to_base_rate) = rates.rates.get(&to) else {
            return Err(ExchangeError::UnsupportedCurrency(to));
        };

        let rate = to_base_rate / base_rate;

        let amount = from.amount * rate;

        Ok(Currency { code: to, amount })
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

    assert_eq!(currency.amount, 1.258506)
}
