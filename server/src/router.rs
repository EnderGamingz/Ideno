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
    let update_contact_info = auth::profile::contact_information::update_contact_information;

    let get_certifications = auth::profile::certification::get_certifications;
    let add_certification = auth::profile::certification::add_certification;
    let delete_certification = auth::profile::certification::delete_certification;
    let update_certification = auth::profile::certification::update_certification;

    let get_educations = auth::profile::education::get_educations;
    let add_education = auth::profile::education::add_education;
    let update_education = auth::profile::education::update_education;
    let delete_education = auth::profile::education::delete_education;

    let get_experiences = auth::profile::experience::get_experiences;
    let add_experience = auth::profile::experience::add_experience;
    let update_experience = auth::profile::experience::update_experience;
    let delete_experience = auth::profile::experience::delete_experience;

    let profile_routes = Router::new()
        .route("/", patch(update_profile))
        .route(
            "/contact-information",
            get(get_contact_info)
                .post(add_contact_info)
                .delete(delete_contact_info)
                .patch(update_contact_info),
        )
        .route(
            "/certification",
            get(get_certifications)
                .post(add_certification)
                .delete(delete_certification)
                .patch(update_certification),
        )
        .route(
            "/education",
            get(get_educations)
                .post(add_education)
                .delete(delete_education)
                .patch(update_education),
        )
        .route(
            "/experience",
            get(get_experiences)
                .post(add_experience)
                .delete(delete_experience)
                .patch(update_experience),
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
