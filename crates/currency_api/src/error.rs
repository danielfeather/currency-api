use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FieldError {
    pub field: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub enum ApiErrorType {
    #[serde(rename = "https://currency.lancastrian.dev/probs/bad-request")]
    BadRequest(Vec<FieldError>),
    #[serde(rename = "https://currency.lancastrian.dev/probs/service-unavailable")]
    ServiceUnavailable,
    #[serde(rename = "https://currency.lancastrian.dev/probs/unsupported-currency")]
    UnsupportedCurrency,
}

impl ApiErrorType {
    fn get_status_code(&self) -> StatusCode {
        match self {
            ApiErrorType::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiErrorType::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            ApiErrorType::UnsupportedCurrency => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    #[serde(rename = "type")]
    error_type: ApiErrorType,
}

impl ApiError {
    pub fn new(error_type: ApiErrorType) -> Self {
        Self { error_type }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.error_type.get_status_code();

        let mut res = Json(self).into_response();

        *res.status_mut() = status_code;

        res
    }
}

#[derive(Debug, Serialize)]
pub struct ApiErrorDetailResponse {}

/// Route handler for returning information about the reported error
pub fn get_error_info() {}
