use crate::todo::Todo;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct GetTodoResponse {
    todo: Todo,
}

#[utoipa::path(
    get,
    path = "/todos/{todo_id}",
    responses(
        (status = 200, description = "Get one todo successfully", body = GetTodoResponse)
    ),
    tag = "Todo",
)]
pub async fn get_todo(Path(todo_id): Path<i32>) -> (StatusCode, Json<GetTodoResponse>) {
    let todo = Todo {
        id: todo_id,
        title: format!("Todo No.{}", todo_id),
        done: false,
    };

    let response: GetTodoResponse = GetTodoResponse { todo };
    (StatusCode::OK, Json(response))
}
