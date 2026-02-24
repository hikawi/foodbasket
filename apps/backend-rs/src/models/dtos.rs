use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MessageResponse {
    pub message: String,
}
