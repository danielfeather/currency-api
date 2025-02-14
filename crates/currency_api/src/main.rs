use chrono::{DateTime, Local};
use serde;
use std::collections::HashMap;

use axum::{routing::get, Json, Router};

use currency_core::providers::open_exchange_rates::Rates;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[axum::debug_handler]
async fn root() -> Json<Rates> {
    Json(Rates {
        disclaimer: "".to_string(),
        license: "".to_string(),
        timestamp: Local::now().to_utc(),
        base: currency_core::CurrencyCode::new("GBP"),
        rates: HashMap::default(),
    })
}

// async fn exchange() -> Json<>
