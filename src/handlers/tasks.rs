use crate::{
    dto::{CreateTaskRequest, UpdateTaskRequest, UpdateTaskStatusRequest},
    error::{AppError, Result},
    models::Task,
    state::AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::{Request, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use serde::Deserialize;
use sqlx::query_as;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize)]
pub struct TaskFilters {
    status: Option<String>,
    priority: Option<String>,
}

/// Get all tasks for the authenticated user
#[utoipa::path(
    get,
    path = "/api/tasks",
    params(
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("priority" = Option<String>, Query, description = "Filter by priority")
    ),
    responses(
        (status = 200, description = "List of tasks", body = Vec<Task>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "tasks",
    security(("bearer_auth" = []))
)]
pub async fn get_tasks(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Query(filters): Query<TaskFilters>,
) -> Result<Json<Vec<Task>>> {
    let repo_filters = crate::repositories::task_repository::TaskFilters {
        status: filters.status,
        priority: filters.priority,
    };

    let tasks = state.task_repository.find_all(user_id, repo_filters).await?;

    Ok(Json(tasks))
}

/// Get a single task by ID
#[utoipa::path(
    get,
    path = "/api/tasks/{id}",
    params(
        ("id" = Uuid, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task found", body = Task),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "tasks",
    security(("bearer_auth" = []))
)]
pub async fn get_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<Task>> {
    let task = state.task_repository.find_by_id(task_id, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    Ok(Json(task))
}

/// Create a new task
#[utoipa::path(
    post,
    path = "/api/tasks",
    request_body = CreateTaskRequest,
    responses(
        (status = 201, description = "Task created", body = Task),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "tasks",
    security(("bearer_auth" = []))
)]
pub async fn create_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse> {
    payload.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let priority = payload.priority.unwrap_or_else(|| "Medium".to_string());

    let task = state.task_repository.create(
        user_id,
        &payload.title,
        payload.description.as_deref(),
        &priority,
        payload.due_date,
        payload.reminder_time,
    ).await?;

    Ok((StatusCode::CREATED, Json(task)))
}

/// Update a task
#[utoipa::path(
    put,
    path = "/api/tasks/{id}",
    params(
        ("id" = Uuid, Path, description = "Task ID")
    ),
    request_body = UpdateTaskRequest,
    responses(
        (status = 200, description = "Task updated", body = Task),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "tasks",
    security(("bearer_auth" = []))
)]
pub async fn update_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<Json<Task>> {
    payload.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let existing_task = state.task_repository.find_by_id(task_id, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    let task = state.task_repository.update(
        task_id,
        user_id,
        payload.title.as_deref(),
        payload.description.as_deref(),
        payload.status.as_deref(),
        payload.priority.as_deref(),
        payload.due_date,
        payload.reminder_time,
    ).await?;

    Ok(Json(task))
}

/// Delete a task
#[utoipa::path(
    delete,
    path = "/api/tasks/{id}",
    params(
        ("id" = Uuid, Path, description = "Task ID")
    ),
    responses(
        (status = 204, description = "Task deleted"),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "tasks",
    security(("bearer_auth" = []))
)]
pub async fn delete_task(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(task_id): Path<Uuid>,
) -> Result<StatusCode> {
    let rows_affected = state.task_repository.delete(task_id, user_id).await?;

    if rows_affected == 0 {
        return Err(AppError::NotFound("Task not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Update task status
#[utoipa::path(
    patch,
    path = "/api/tasks/{id}/status",
    params(
        ("id" = Uuid, Path, description = "Task ID")
    ),
    request_body = UpdateTaskStatusRequest,
    responses(
        (status = 200, description = "Status updated", body = Task),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "tasks",
    security(("bearer_auth" = []))
)]
pub async fn update_task_status(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<UpdateTaskStatusRequest>,
) -> Result<Json<Task>> {
    let task = state.task_repository.update_status(task_id, user_id, &payload.status)
        .await?
    .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    Ok(Json(task))
}
