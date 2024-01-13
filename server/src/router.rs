use axum::routing::{delete, get, patch, post};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::Level;

use crate::routes::api::{auth, user};
use crate::AppState;

fn create_auth_routes() -> Router<AppState> {
    let auth = auth::auth::auth;
    let login = auth::login::login;
    let register = auth::register::register;
    let logout = auth::logout::logout;
    let update_profile = auth::profile::index::update_profile;
    let update_account = auth::account::update_account;
    let update_password = auth::account::update_password;
    let delete_account = auth::account::delete_account;
    let get_contact_info = auth::profile::contact_information::get_contact_information;
    let add_contact_info = auth::profile::contact_information::add_contact_information;
    let delete_contact_info = auth::profile::contact_information::delete_contact_information;

    let profile_routes = Router::new().route("/", patch(update_profile)).route(
        "/contact-information",
        get(get_contact_info)
            .post(add_contact_info)
            .delete(delete_contact_info),
    );

    // /auth
    Router::new()
        .route("/", get(auth))
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/logout", get(logout))
        .route("/account", patch(update_account).delete(delete_account))
        .route("/password", patch(update_password))
        .nest("/profile", profile_routes)
}

fn create_user_routes() -> Router<AppState> {
    let delete_user = user::delete_user;
    Router::new().route("/", delete(delete_user))
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
