use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "FoodBasket API",
        description = "Description for the FoodBasket's platform-wide API.",
        contact(
            name = "Nguyệt Ánh",
            url = "https://www.luny.dev",
            email = "hello@luny.dev"
        ),
        license(name = "Apache-2.0", identifier = "Apache-2.0"),
        version = "v1",
    ),
    servers((url = "/v1", description = "Current version of the API")),
    paths(
        crate::routes::health::handler::health_check, 
        crate::routes::auth::handler::login,
        crate::routes::auth::handler::register,
        crate::routes::auth::handler::logout,
        crate::routes::auth::handler::get_me,
        crate::routes::tenants::handler::get_tenants,
        crate::routes::tenants::handler::create_tenant,
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDocs;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "session_id",
            SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("session_id"))),
        )
    }
}
