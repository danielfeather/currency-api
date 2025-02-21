use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use currency_core::{Currency, CurrencyCode, Exchange};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

use crate::{error::ApiError, ApiState};

pub fn router() -> Router<Arc<ApiState>> {
    return Router::new()
        .route("/exchange", post(exchange))
        .route("/rates", get(get_rates))
        .route("/rates", post(post_rates))
        .route("/currencies", get(get_currencies));
}

#[derive(Debug, FromRow)]
struct TimestampCurrencyCount {
    timestamp: u32,
    count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostNewRates {
    pub timestamp: chrono::DateTime<Utc>,
    pub base: CurrencyCode,
    pub rates: HashMap<CurrencyCode, u32>,
}

impl Default for PostNewRates {
    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            base: CurrencyCode("USD".to_string()),
            rates: HashMap::new(),
        }
    }
}

/// Post route handler for handling upload of new rates
async fn post_rates(
    State(api_state): State<Arc<ApiState>>,
    Json(body): Json<PostNewRates>,
) -> Result<StatusCode, ApiError> {
    let Ok(check) = sqlx::query_as::<_, TimestampCurrencyCount>(
        "SELECT timestamp, COUNT(*) as count FROM rates",
    )
    .fetch_all(&api_state.pool)
    .await
    else {
        return Ok(StatusCode::OK);
    };

    for (code, rate) in body.rates.iter() {
        let result = sqlx::query("INSERT INTO rates (timestamp, code, rate) VALUES (?, ?, ?)")
            .bind(body.timestamp.timestamp())
            .bind(&code.0)
            .bind(rate)
            .execute(&api_state.pool)
            .await;
    }

    Ok(StatusCode::ACCEPTED)
}

#[derive(Debug, FromRow, Serialize)]
struct DbRate {
    code: String,
    rate: u32,
}

#[axum::debug_handler]
async fn get_rates(State(api_state): State<Arc<ApiState>>) -> Result<Json<Vec<DbRate>>, ApiError> {
    let Ok(rates) = sqlx::query_as::<_, DbRate>("SELECT code, rate FROM rates")
        .fetch_all(&api_state.pool)
        .await
    else {
        return Err(ApiError::new(
            crate::error::ApiErrorType::ServiceUnavailable,
        ));
    };

    Ok(Json(rates))
}

#[derive(Debug, Serialize, FromRow)]
struct DbCurrency {
    code: String,
    name: String,
}

/// Route handler for retrieving all the supported currencies
async fn get_currencies(
    State(api_state): State<Arc<ApiState>>,
) -> Result<Json<Vec<DbCurrency>>, ApiError> {
    let Ok(currencies) = sqlx::query_as::<_, DbCurrency>("SELECT * FROM currencies")
        .fetch_all(&api_state.pool)
        .await
    else {
        return Err(ApiError::new(
            crate::error::ApiErrorType::UnsupportedCurrency,
        ));
    };

    Ok(Json(currencies))
}

#[derive(Debug, Deserialize)]
struct ExchangeRequest {
    from: Currency,
    to: CurrencyCode,
}

async fn exchange(
    State(api_state): State<Arc<ApiState>>,
    Json(body): Json<ExchangeRequest>,
) -> Result<Json<Currency>, ApiError> {
    let mut exchange = Exchange::new();

    exchange
        .from(Currency::new(body.from.code, body.from.amount))
        .to(body.to);

    let Ok(exchange) = exchange.exchange(&api_state.rates.clone()) else {
        return Err(ApiError::new(
            crate::error::ApiErrorType::UnsupportedCurrency,
        ));
    };

    Ok(Json(exchange))
}
