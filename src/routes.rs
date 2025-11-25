use crate::{
    dto::*,
    handlers::{
        self,
        auth::{register, login, google_login, google_callback},
        tasks::{get_tasks, get_task, create_task, update_task, delete_task, update_task_status},
        notifications::{get_notifications, notification_stream, mark_notification_read, delete_notification, update_notification_preferences},
    },
    middleware::auth_middleware,
    models::*,
    state::AppState,
};
use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::auth::register,
        crate::handlers::auth::login,
        crate::handlers::auth::google_login,
        crate::handlers::auth::google_callback,
        crate::handlers::tasks::get_tasks,
        crate::handlers::tasks::get_task,
        crate::handlers::tasks::create_task,
        crate::handlers::tasks::update_task,
        crate::handlers::tasks::delete_task,
        crate::handlers::tasks::update_task_status,
        crate::handlers::notifications::get_notifications,
        crate::handlers::notifications::notification_stream,
        crate::handlers::notifications::mark_notification_read,
        crate::handlers::notifications::delete_notification,
        crate::handlers::notifications::update_notification_preferences,
    ),
    components(
        schemas(
            RegisterRequest,
            LoginRequest,
            AuthResponse,
            CreateTaskRequest,
            UpdateTaskRequest,
            UpdateTaskStatusRequest,
            UpdateNotificationPreferencesRequest,
            User,
            UserResponse,
            Task,
            TaskStatus,
            TaskPriority,
            Notification,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "tasks", description = "Task management endpoints"),
        (name = "notifications", description = "Notification endpoints")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            )
        }
    }
}

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Public routes (no auth required)
    let auth_routes = Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/google", get(handlers::google_login))
        .route("/google/callback", get(handlers::google_callback));

    // Protected routes (auth required)
    let task_routes = Router::new()
        .route("/", get(handlers::get_tasks).post(handlers::create_task))
        .route(
            "/:id",
            get(handlers::get_task)
                .put(handlers::update_task)
                .delete(handlers::delete_task),
        )
        .route("/:id/status", patch(handlers::update_task_status))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let notification_routes = Router::new()
        .route("/", get(handlers::get_notifications))
        .route("/stream", get(handlers::notification_stream))
        .route("/:id/read", patch(handlers::mark_notification_read))
        .route("/:id", delete(handlers::delete_notification))
        .route(
            "/preferences",
            put(handlers::update_notification_preferences),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    // Combine all routes
    let api_routes = Router::new()
        .nest("/auth", auth_routes)
        .nest("/tasks", task_routes)
        .nest("/notifications", notification_routes);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_routes)
        .layer(cors)
        .with_state(state)
}
