mod reservation;
mod config;


use std::sync::Arc;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Router;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[cfg(feature = "local")]
use config::local::init as config_init;

use crate::reservation::timelines::make_reservation;

pub struct AppState {
    db: MySqlPool,
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config_init();

    let pool = create_db_connection(&config.database_url).await
        .unwrap_or_else(|_| std::process::exit(1));

    let hc_router = Router::new().route("/", get(health_check));
    let reservasion_router = Router::new()
        .route("/", post(make_reservation));

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/hc", hc_router)
        .nest("/reserve", reservasion_router)
        .with_state(Arc::new(AppState { db: pool.clone() }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[derive(OpenApi)]
#[openapi(
    paths(
        reservation::timelines::make_reservation,
    ),
    components(schemas(
        reservation::data_models::api_schemas::ReservationRequest,
        reservation::data_models::api_schemas::ReservationResponse,
        reservation::data_models::entities::CardDetails,
    )),
    tags((name = "Reservation"))
)]
struct ApiDoc;

async fn create_db_connection(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await;

    match pool {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            Ok(pool)
        },
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            Err(err)
        }
    }
}
