use std::{collections::HashMap, env};

use currency_core::{
    providers::open_exchange_rates::{Rates, BASE_URL},
    CurrencyCode, Exchange,
};
use thiserror::Error;
use ureq::BodyReader;

fn main() {
    let latest = fetch_latest_rates().expect("Failed to fetch rates");

    let mut rates: HashMap<CurrencyCode, u32> = HashMap::new();

    for (code, rate) in latest.rates.iter() {
        rates.insert(code.clone(), (rate * 100.0).round() as u32);
    }

    // ureq::post("http://localhost:3000/v1/rates")
}

#[derive(Debug, Error)]
pub enum FetchRatesError {
    #[error("Response was in a format that isn't recognised")]
    UnexpectedResponse,
}

fn fetch_latest_rates() -> Result<Rates, FetchRatesError> {
    let token = env::var("TOKEN").expect("Missing environment variable, TOKEN");

    let mut res = ureq::get(format!("{}/latest.json", BASE_URL))
        .header("Authorization", format!("Token {}", token))
        .call()
        .map_err(|_| FetchRatesError::UnexpectedResponse)?;

    serde_json::from_reader::<BodyReader, Rates>(res.body_mut().as_reader())
        .map_err(|_| FetchRatesError::UnexpectedResponse)
}
