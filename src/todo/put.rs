use crate::todo::{Todo, UpdateTodo};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PutTodoRequest {
    todo: UpdateTodo,
}

#[derive(Serialize, ToSchema)]
pub struct PutTodoResponse {
    todo: Todo,
}

#[utoipa::path(
    put,
    path = "/todos/{todo_id}",
    request_body = PutTodoRequest,
    responses(
        (status = 200, description = "Todo item updated successfully", body = PutTodoResponse)
    ),
    tag = "Todo",
)]
pub async fn put_todo(
    Path(todo_id): Path<i32>,
    Json(source): Json<PutTodoRequest>,
) -> (StatusCode, Json<PutTodoResponse>) {
    let request: PutTodoRequest = source.into();

    let response = PutTodoResponse {
        todo: Todo {
            id: todo_id,
            title: request.todo.title,
            done: request.todo.done,
        },
    };

    (StatusCode::OK, Json(response))
}
