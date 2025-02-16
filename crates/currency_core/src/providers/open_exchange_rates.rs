use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::CurrencyCode;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rates {
    pub disclaimer: String,
    pub license: String,
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub base: CurrencyCode,
    pub rates: HashMap<CurrencyCode, f32>,
}

pub const BASE_URL: &'static str = "https://openexchangerates.org/api";
