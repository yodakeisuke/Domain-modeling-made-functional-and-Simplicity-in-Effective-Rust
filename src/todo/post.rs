use crate::todo::{NewTodo, Todo};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PostTodoRequest {
    todo: NewTodo,
}

#[derive(Serialize, ToSchema)]
pub struct PostTodoResponse {
    todo: Todo,
}

#[utoipa::path(
    post,
    path = "/todos",
    request_body = PostTodoRequest,
    responses(
        (status = 201, description = "Todo item created successfully", body = PostTodoResponse)
    ),
    tag = "Todo",
)]
pub async fn post_todo(
    Json(source): Json<PostTodoRequest>,
) -> (StatusCode, Json<PostTodoResponse>) {
    let request: PostTodoRequest = source.into();

    let response = PostTodoResponse {
        todo: Todo {
            id: 100,
            title: request.todo.title,
            done: false,
        },
    };

    (StatusCode::CREATED, Json(response))
}
