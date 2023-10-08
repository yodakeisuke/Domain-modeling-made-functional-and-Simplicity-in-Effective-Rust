mod todo;

use crate::todo::delete::delete_todo;
use crate::todo::get::get_todo;
use crate::todo::list::list_todos;
use crate::todo::post::post_todo;
use crate::todo::put::put_todo;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let hc_router = Router::new().route("/", get(health_check));
    let todo_router = Router::new()
        .route("/", get(list_todos).post(post_todo))
        .route("/:todo_id", get(get_todo).put(put_todo).delete(delete_todo));

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/hc", hc_router)
        .nest("/todos", todo_router);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[derive(OpenApi)]
#[openapi(
    paths(
        todo::list::list_todos,
        todo::get::get_todo,
        todo::post::post_todo,
        todo::put::put_todo,
        todo::delete::delete_todo
    ),
    components(schemas(
        todo::Todo,
        todo::NewTodo,
        todo::UpdateTodo,
        todo::list::ListTodoResponse,
        todo::get::GetTodoResponse,
        todo::post::PostTodoRequest,
        todo::post::PostTodoResponse,
        todo::put::PutTodoRequest,
        todo::put::PutTodoResponse,
    )),
    tags((name = "Todo"))
)]
struct ApiDoc;
