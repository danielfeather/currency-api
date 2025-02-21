//! # Currency API
//!
//! Web API for aggregating exchange rates and providing currency exchange services

use chrono::Local;
use sqlx::{Pool, Sqlite};
use std::{collections::HashMap, sync::Arc};

use axum::{routing::get, Json, Router};

use currency_core::{providers::open_exchange_rates::Rates, CurrencyCode};

mod error;
mod v1;

struct ApiState {
    pool: Pool<Sqlite>,
    rates: Rates,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = sqlx::sqlite::SqlitePool::connect("sqlite:crates/currency_api/data/currency_api.db")
        .await
        .unwrap();

    let mut rates = HashMap::new();

    rates.insert(CurrencyCode("AZN".to_string()), 1.7);
    rates.insert(CurrencyCode("GBP".to_string()), 0.794593);
    rates.insert(CurrencyCode("USD".to_string()), 1.0);

    let api_state = Arc::new(ApiState {
        pool,
        rates: Rates {
            disclaimer: "Usage subject to terms: https://openexchangerates.org/terms".to_string(),
            license: "https://openexchangerates.org/license".to_string(),
            timestamp: Local::now().to_utc(),
            base: CurrencyCode("USD".to_string()),
            rates,
        },
    });

    let app: Router = Router::new()
        .route("/", get(root))
        .nest("/v1", v1::router())
        .with_state(api_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<Rates> {
    Json(Rates {
        disclaimer: "".to_string(),
        license: "".to_string(),
        timestamp: Local::now().to_utc(),
        base: currency_core::CurrencyCode::new("GBP"),
        rates: HashMap::default(),
    })
}

async fn upload() {}

// async fn exchange() -> Json<>
