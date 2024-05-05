use axum::http::StatusCode;
use axum::Json;
use crate::reservation::data_models::api_schemas::{ReservationRequest, ReservationResponse};
use crate::reservation::actions::validation_tasks::accept_reservation;

#[utoipa::path(
    post,
    path = "/reserve",
    request_body = ReservationRequest,
    responses(
        (status = 201, description = "Reservasion made successfully", body = ReservationResponse)
    ),
    tag = "Reservation",
)]
pub async fn make_reservation(
    Json(request): Json<ReservationRequest>,
) -> (StatusCode, Json<ReservationResponse>) {
    let _res = accept_reservation(request);

    let response = ReservationResponse {
        reservation_id: "123456".to_string(),
        status: "Confirmed".to_string(),
        restaurant_name: "Gourmet Steakhouse".to_string(),
        reservation_type: "SeatsOnly".to_string(),
        reserved_date_time: "2021-09-01T19:00:00Z".to_string(),
        message: "Your reservation is confirmed.".to_string(),
        total_amount: Some(59.99),
    };

    (StatusCode::CREATED, Json(response))
}
