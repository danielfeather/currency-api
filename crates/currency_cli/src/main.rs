use std::env;

use currency_core::providers::open_exchange_rates::{Rates, BASE_URL};
use thiserror::Error;
use ureq::BodyReader;

fn main() {
    let latest = fetch_latest_rates().expect("Failed to fetch rates");

    let base = for (code, rate) in latest.rates.iter() {};
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
