use crate::todo::Todo;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ListTodoResponse {
    todos: Vec<Todo>,
}

#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "List all todos successfully", body = ListTodoResponse)
    ),
    tag = "Todo",
)]
pub async fn list_todos() -> (StatusCode, Json<ListTodoResponse>) {
    let todos = vec![
        Todo {
            id: 0,
            title: "Todo 1".to_string(),
            done: false,
        },
        Todo {
            id: 1,
            title: "Todo 2".to_string(),
            done: true,
        },
        Todo {
            id: 2,
            title: "Todo 3".to_string(),
            done: false,
        },
    ];

    let response: ListTodoResponse = ListTodoResponse { todos };
    (StatusCode::OK, Json(response))
}
