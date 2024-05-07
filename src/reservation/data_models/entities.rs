use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use crate::reservation::data_models::values::{NumberOfGuests, RestaurantId, UserId, CourseName, SpecialRequests};


// entities
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ValidatedReservationRequest {
    user_id: UserId,
    received_timestamp: DateTime<Utc>,
    restaurant_id: RestaurantId,
    reservation_type: ReservationType,
    payment_option: PaymentOption,
    special_requests: SpecialRequests,
}


// items
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub enum ReservationType {
    SeatsOnly {
        number_of_guests: NumberOfGuests,
        start_time: DateTime<Utc>,
    },
    Course {
        number_of_guests: NumberOfGuests,
        start_time: DateTime<Utc>,
        course_name: CourseName,
    },
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub enum PaymentOption {
    Card(CardDetails),
    Cash,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct CardDetails {
    card_number: String,
    expiry_date: String,
    cvv: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub enum ReservationStatus {
    Confirmed,
    Failed,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub enum PaymentStatus {
    Paid,
    Unpaid,
}

impl ValidatedReservationRequest {
    pub fn new(
        user_id: UserId,
        received_timestamp: DateTime<Utc>,
        restaurant_id: RestaurantId,
        reservation_type: ReservationType,
        payment_option: PaymentOption,
        special_requests: SpecialRequests,
    ) -> Self {
        ValidatedReservationRequest {
            user_id,
            received_timestamp,
            restaurant_id,
            reservation_type,
            payment_option,
            special_requests,
        }
    }
}
