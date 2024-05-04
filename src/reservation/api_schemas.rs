use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

// make_reservation
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ReservationRequest {
    pub restaurant_id: String,
    pub reservation_type: String,
    pub request_date_time: String,
    pub number_of_guests: u32,
    pub customer_details: CustomerDetails,
    pub payment_method: String,
    pub special_requests: Option<String>,
    pub card_details: Option<CardDetails>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ReservationResponse {
    pub reservation_id: String,
    pub status: String,
    pub restaurant_name: String,
    pub reservation_type: String,
    pub reserved_date_time: String,
    pub customer_details: CustomerDetails,
    pub message: String,
    pub total_amount: Option<f64>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct CardDetails {
    card_number: String,
    expiry_date: String,
    cvv: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct CustomerDetails {
    pub name: String,
    pub email: String,
    pub phone: String,
}
