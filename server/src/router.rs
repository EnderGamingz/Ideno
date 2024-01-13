use axum::routing::{delete, get, patch, post};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::Level;

use crate::{routes, AppState};

fn create_auth_routes() -> Router<AppState> {
    let auth = routes::api::auth::auth::auth;
    let login = routes::api::auth::login::login;
    let register = routes::api::auth::register::register;
    let logout = routes::api::auth::logout::logout;
    let update_profile = routes::api::auth::profile::update_profile;
    let update_account = routes::api::auth::account::update_account;
    let update_password = routes::api::auth::account::update_password;

    // /auth
    Router::new()
        .route("/", get(auth))
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/logout", get(logout))
        .route("/profile", patch(update_profile))
        .route("/account", patch(update_account))
        .route("/password", patch(update_password))
}

fn create_user_routes() -> Router<AppState> {
    let get_users = routes::api::user::get_users;
    let delete_user = routes::api::user::delete_user;
    Router::new()
        .route("/all", get(get_users))
        .route("/", delete(delete_user))
}

pub fn router(
    cors: CorsLayer,
    session_layer: SessionManagerLayer<MemoryStore>,
    state: AppState,
) -> Router {
    let auth_routes = create_auth_routes();
    let user_routes = create_user_routes();

    let api_router = Router::new()
        .nest("/auth", auth_routes)
        .nest("/user", user_routes);

    Router::new()
        .nest("/api/v1", api_router)
        .layer(session_layer)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state)
}
