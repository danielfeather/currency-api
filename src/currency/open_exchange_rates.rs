use std::collections::HashMap;

use serde::Deserialize;

use super::Currency;

#[derive(Debug, Deserialize)]
pub struct Rates {
    disclaimer: String,
    license: String,
    timestamp: usize,
    base: Currency,
    rates: HashMap<Currency, f32>,
}
