use axum::routing::{delete, get, patch, post};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::Level;

use crate::routes::api::{auth, profile};
use crate::AppState;

/// Creates the authentication routes.
///
/// The function sets up the routes for authentication related operations like
/// login, registration, logout, updating account information, and updating passwords.
///
/// # Returns
///
/// A `Router<AppState>` instance configured to handle the authentication routes.
///
fn create_auth_routes() -> Router<AppState> {
    let auth = auth::auth::auth;
    let login = auth::login::login;
    let register = auth::register::register;
    let logout = auth::logout::logout;
    let update_account = auth::account::update_account;
    let update_password = auth::account::update_password;
    let delete_account = auth::account::delete_account;

    // /auth
    Router::new()
        .route("/", get(auth))
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/logout", get(logout))
        .route("/account", patch(update_account).delete(delete_account))
        .route("/password", patch(update_password))
        .nest("/profile", create_auth_profile_routes())
        .nest("/admin", create_auth_admin_routes())
}

/// Create routes for the authentication profile.
///
/// This function creates routes for managing the user's profile in the authentication system.
/// It includes routes for getting, updating, adding, and deleting various profile information like contact information,
/// certifications, educations, and experiences.
///
/// # Returns
///
/// A `Router<AppState>` instance configured to handle the authentication profile routes.
///
fn create_auth_profile_routes() -> Router<AppState> {
    let get_profile = auth::profile::index::get_profile;
    let update_profile = auth::profile::index::update_profile;

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

    Router::new()
        .route("/", get(get_profile).patch(update_profile))
        .route(
            "/contact-information",
            get(get_contact_info).post(add_contact_info),
        )
        .route(
            "/contact-information/:id",
            delete(delete_contact_info).patch(update_contact_info),
        )
        .route(
            "/certification",
            get(get_certifications).post(add_certification),
        )
        .route(
            "/certification/:id",
            delete(delete_certification).patch(update_certification),
        )
        .route("/education", get(get_educations).post(add_education))
        .route(
            "/education/:id",
            delete(delete_education).patch(update_education),
        )
        .route("/experience", get(get_experiences).post(add_experience))
        .route(
            "/experience/:id",
            delete(delete_experience).patch(update_experience),
        )
}



/// Creates the authentication admin routes.
///
/// The function initializes various route handlers for managing users, certifications, education,
/// experience, and contact information within the admin panel.
///
/// # Returns
///
/// Returns a `Router<AppState>` instance configured to handle the authentication admin routes.
///
fn create_auth_admin_routes() -> Router<AppState> {
    let get_all_users = auth::admin::user::admin_get_users;
    let get_user = auth::admin::user::admin_get_user;
    let delete_user = auth::admin::user::admin_delete_user;
    let update_user = auth::admin::user::admin_update_user;

    let delete_certification = auth::admin::certification::admin_delete_certification;
    let delete_education = auth::admin::education::admin_delete_education;
    let delete_experience = auth::admin::experience::admin_delete_experience;
    let delete_contact_information =
        auth::admin::contact_information::admin_delete_contact_information;

    Router::new()
        .route("/users", get(get_all_users))
        .route(
            "/users/:id",
            get(get_user).delete(delete_user).patch(update_user),
        )
        .route("/certification/:id", delete(delete_certification))
        .route("/education/:id", delete(delete_education))
        .route("/experience/:id", delete(delete_experience))
        .route(
            "/contact-information/:id",
            delete(delete_contact_information),
        )
}

/// Creates the router for public profile routes.
///
/// This function constructs a router and sets up routes for various endpoints related to public profiles.
///
/// # Returns
///
/// A `Router<AppState>` instance configured to handle the public profile routes.
///
fn create_public_profile_routes() -> Router<AppState> {
    let get_public_profile = profile::index::get_public_profile;
    let get_public_certifications = profile::certification::get_public_certifications;
    let get_public_educations = profile::educations::get_public_educations;
    let get_public_experiences = profile::experience::get_public_experiences;
    let get_public_contact_information =
        profile::contact_information::get_public_contact_information;

    return Router::new()
        .route("/:id", get(get_public_profile))
        .route("/:id/certifications", get(get_public_certifications))
        .route("/:id/educations", get(get_public_educations))
        .route("/:id/experiences", get(get_public_experiences))
        .route(
            "/:id/contact-information",
            get(get_public_contact_information),
        );
}

/// This function creates a new router with the specified configuration.
///
/// # Arguments
///
/// * `cors` - A `CorsLayer` instance to handle CORS in the routing.
/// * `session_layer` - A `SessionManagerLayer<MemoryStore>` to manage sessions in the application.
/// * `state` - An `AppState` instance representing the application state.
///
/// The function establishes various routes for "/auth" and "/profile" which are nested under "/api/v1".
/// It applies the session_layer and cors middleware layers to the router along with tracing layer for logging.
/// It also injects the application's state to the router.
///
/// # Returns
///
/// It returns an instance of a configured `Router`.
///
/// # Example
///
/// ```rust
/// let cors = CorsLayer::new();
/// let session_layer = SessionManagerLayer::new(MemoryStore::default())
/// let state = AppState { ... };
///
/// let router = router(cors, session_layer, state);
/// ```
///
pub fn router(
    cors: CorsLayer,
    session_layer: SessionManagerLayer<MemoryStore>,
    state: AppState,
) -> Router {
    let api_router = Router::new()
        .nest("/auth", create_auth_routes())
        .nest("/profile", create_public_profile_routes());

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
