mod reservation;

use crate::reservation::workflows::make_reservation;

use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let hc_router = Router::new().route("/", get(health_check));
    let reservasion_router = Router::new()
        .route("/", post(make_reservation));

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/hc", hc_router)
        .nest("/reserve", reservasion_router);

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
        reservation::workflows::make_reservation,
    ),
    components(schemas(
        reservation::api_schemas::ReservationRequest,
        reservation::api_schemas::ReservationResponse,
        reservation::api_schemas::CardDetails,
        reservation::api_schemas::CustomerDetails,
    )),
    tags((name = "Reservation"))
)]
struct ApiDoc;
