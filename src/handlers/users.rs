use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use validator::Validate;

use crate::{
    dto::{UpdateProfileRequest, UserStatsResponse},
    error::{AppError, Result},
    middleware::auth::AuthUser,
    models::UserResponse,
    state::AppState,
};

/// Get current user profile
#[utoipa::path(
    get,
    path = "/api/users/me",
    tag = "users",
    responses(
        (status = 200, description = "User profile retrieved successfully", body = UserResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_current_user(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<impl IntoResponse> {
    let user = state
        .user_repository
        .find_by_id(user_id)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    Ok((StatusCode::OK, Json(UserResponse::from(user))))
}

/// Update current user profile
#[utoipa::path(
    put,
    path = "/api/users/me",
    tag = "users",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = UserResponse),
        (status = 400, description = "Invalid input"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_current_user(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse> {
    payload.validate()?;

    let user = state
        .user_repository
        .update_profile(
            user_id,
            payload.username,
            payload.bio,
            payload.theme,
            payload.avatar_url,
        )
        .await?;

    Ok((StatusCode::OK, Json(UserResponse::from(user))))
}

/// Get user task statistics
#[utoipa::path(
    get,
    path = "/api/users/me/stats",
    tag = "users",
    responses(
        (status = 200, description = "User statistics retrieved successfully", body = UserStatsResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_user_stats(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<impl IntoResponse> {
    let (
        total_tasks,
        pending_tasks,
        in_progress_tasks,
        completed_tasks,
        archived_tasks,
        low_priority_tasks,
        medium_priority_tasks,
        high_priority_tasks,
        urgent_priority_tasks,
    ) = state.task_repository.get_user_stats(user_id).await?;

    let stats = UserStatsResponse {
        total_tasks,
        pending_tasks,
        in_progress_tasks,
        completed_tasks,
        archived_tasks,
        low_priority_tasks,
        medium_priority_tasks,
        high_priority_tasks,
        urgent_priority_tasks,
    };

    Ok((StatusCode::OK, Json(stats)))
}
