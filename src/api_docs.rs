use {
    crate::features::{
        auth::{
            dto::{
                LoginRequest,
                RegisterRequest,
            },
            handlers::*,
        },
        groups::{
            dto::*,
            handlers::*,
        },
        users::{
            dto::*,
            handlers::*,
        },
    },
    serde::Serialize,
    utoipa::{
        openapi::security::{
            Http,
            HttpAuthScheme,
            SecurityScheme,
        },
        Modify,
        OpenApi,
    },
};

#[derive(Debug, Serialize)]
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components: &mut utoipa::openapi::Components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        )
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        login,
        register,
        get_user_by_id,
        get_all_user,
        update_user,
        delete_user,
        update_avatar,
        create_group,
        get_group_by_id,
        get_all_groups_of_user,
        create_user
    ),
    components(
        schemas(
            LoginRequest, 
            RegisterRequest, 
            UserResponse, 
            UpdateUserRequest,
            CreateGroup,
            GroupResponse,
            CreateUserRequest
        )
    ),
    tags(
        (name = "Users", description = "Endpoints related to user management"),
        (name = "Groups", description = "Endpoints related to group management"),
        (name = "Authentication", description = "Endpoints for authentication")
    ),
    modifiers(&SecurityAddon),
    security(
        ("bearerAuth" = []) // Define the global security scheme
    )
)]
pub struct ApiDoc;
