use axum::response::IntoResponse;

use crate::{
    api::responses::ErrorResponse,
    routes::{
        auth::AuthError, middlewares::MiddlewareError, staff::StaffError, tenants::TenantError,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Tenant(#[from] TenantError),

    #[error(transparent)]
    Staff(#[from] StaffError),

    #[error(transparent)]
    Middleware(#[from] MiddlewareError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message) = match self {
            Self::Auth(e) => e.extract(),
            Self::Staff(e) => e.extract(),
            Self::Tenant(e) => e.extract(),
            Self::Middleware(e) => e.extract(),
        };

        tracing::error!(code = %code, status = %status.as_u16(), message);
        ErrorResponse::new(status, &code, &message).into_response()
    }
}
