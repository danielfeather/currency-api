use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use currency_core::{Currency, CurrencyCode, Exchange};
use serde::Deserialize;

use crate::ApiState;

pub fn router() -> Router<Arc<ApiState>> {
    return Router::new().route("/rates", get(rates));
}

#[derive(Debug, Deserialize)]
struct RatesQuery {
    from: CurrencyCode,
    to: CurrencyCode,
}

impl Default for RatesQuery {
    fn default() -> Self {
        Self {
            from: CurrencyCode("USD".to_string()),
            to: CurrencyCode("GBP".to_string()),
        }
    }
}

async fn rates(
    State(api_state): State<Arc<ApiState>>,
    Query(query): Query<RatesQuery>,
) -> impl IntoResponse {
    let mut exchange = Exchange::new();

    exchange.from(Currency::new(query.from, 1.0)).to(query.to);

    format!("{}", exchange.exchange(&api_state.rates.clone()).amount)
}
