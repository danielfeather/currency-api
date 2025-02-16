use std::env;

use currency_core::{
    providers::open_exchange_rates::{Rates, BASE_URL},
    Currency, CurrencyCode, Exchange,
};
use thiserror::Error;
use ureq::BodyReader;

fn main() {
    let latest = fetch_latest_rates().expect("Failed to fetch rates");

    for (code, rate) in latest.rates.iter() {
        let mut exchange = Exchange::new();

        let code = code.clone();

        if code != CurrencyCode::new("GBP") {
            continue;
        }

        exchange
            .from(Currency::new(CurrencyCode::new("USD"), 1.0))
            .to(code);

        let currency = exchange.exchange(&latest);

        println!(
            "1.0 USD is {} {}",
            (currency.amount * 100.0) as u32,
            currency.code
        )
    }
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
