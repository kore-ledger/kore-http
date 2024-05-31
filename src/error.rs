use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

// Errors
pub enum Errors {
    State,
    InvalidIP,
    MaxAttemptsOfRequests,

    Kore(String)
}

impl IntoResponse for Errors {
    fn into_response(self) -> Response {
        match self {
			Errors::State => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error: An internal error has occurred"),
            )
                .into_response(),
            Errors::InvalidIP => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error: the IP is invalid"),
            )
                .into_response(),
            Errors::MaxAttemptsOfRequests => (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json("Error: the maximum number of requests attempts for this ip has been reached, please wait before trying again")
            )
                .into_response(),
            Errors::Kore(error) => (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(error)
            )
                .into_response(),
        }
    }
}
