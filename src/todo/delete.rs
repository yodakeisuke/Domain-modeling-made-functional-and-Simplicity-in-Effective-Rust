use axum::extract::Path;
use axum::http::StatusCode;

#[utoipa::path(
    delete,
    path = "/todos/{todo_id}",
    responses(
        (status = 204, description = "Deleted a todo successfully")
    ),
    tag = "Todo",
)]
pub async fn delete_todo(Path(_todo_id): Path<i32>) -> StatusCode {
    StatusCode::NO_CONTENT
}
