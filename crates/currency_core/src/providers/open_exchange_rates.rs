use std::collections::HashMap;

use chrono::DateTime;
use serde::{Deserialize, Serialize};

use crate::Currency;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rates {
    pub disclaimer: String,
    pub license: String,
    pub timestamp: DateTime,
    pub base: Currency,
    pub rates: HashMap<Currency, f32>,
}
