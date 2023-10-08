use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod delete;
pub mod get;
pub mod list;
pub mod post;
pub mod put;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Todo {
    id: i32,
    title: String,
    done: bool,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NewTodo {
    title: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateTodo {
    title: String,
    done: bool,
}
