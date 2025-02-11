use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Currency;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rates {
    pub disclaimer: String,
    pub license: String,
    pub timestamp: time::OffsetDateTime,
    pub base: Currency,
    pub rates: HashMap<Currency, f32>,
}
