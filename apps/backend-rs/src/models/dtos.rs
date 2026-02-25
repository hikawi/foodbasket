use axum::Json;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl MessageResponse {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.into(),
        }
    }

    pub fn json(msg: &str) -> Json<Self> {
        Json(Self::new(msg))
    }
}

impl ErrorResponse {
    pub fn new(err: &str) -> Self {
        Self { error: err.into() }
    }

    pub fn json(msg: &str) -> Json<Self> {
        Json(Self::new(msg))
    }
}
