use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use std::convert::TryFrom;

use crate::reservation::data_models::values::ReservationValueError;
use crate::reservation::data_models::entities::{
    ValidatedReservationRequest, ReservationType, CardDetails, PaymentOption
};

// make_reservation
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ReservationRequest {
    pub user_id: String,
    pub restaurant_id: String,
    pub reservation_type: ReservationTypeEnum,
    pub start_date_time: String,
    pub number_of_guests: u32,
    pub course_name: Option<String>,
    pub payment_option: PaymentOptionRequest,
    pub special_requests: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ReservationResponse {
    pub reservation_id: String,
    pub status: String,
    pub restaurant_name: String,
    pub reservation_type: String,
    pub reserved_date_time: String,
    pub message: String,
    pub total_amount: Option<f64>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub enum ReservationTypeEnum {
    SeatsOnly,
    Course,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct PaymentOptionRequest {
    pub payment_method: PaymentMethodEnum,
    pub card_details: Option<CardDetails>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub enum PaymentMethodEnum {
    Card,
    Cash,
}

impl TryFrom<ReservationRequest> for ValidatedReservationRequest {
    type Error = ReservationValueError;

    fn try_from(request: ReservationRequest) -> Result<Self, Self::Error> {
        let user_id = request.user_id.try_into()?;
        let restaurant_id = request.restaurant_id.try_into()?;
        let received_timestamp = Utc::now();
        let reservation_type = match request.reservation_type {
            ReservationTypeEnum::SeatsOnly => ReservationType::SeatsOnly {
                number_of_guests: request.number_of_guests.try_into()?,
                start_time: DateTime::parse_from_rfc3339(&request.start_date_time)?.with_timezone(&Utc),
            },
            ReservationTypeEnum::Course => ReservationType::Course {
                number_of_guests: request.number_of_guests.try_into()?,
                start_time: DateTime::parse_from_rfc3339(&request.start_date_time)?.with_timezone(&Utc),
                course_name: request.course_name.clone().ok_or(ReservationValueError::MissingCourseName)?.try_into()?,
            },
        };
        let payment_option = match request.payment_option.payment_method {
            PaymentMethodEnum::Card => match request.payment_option.card_details {
                Some(card_details) => PaymentOption::Card(card_details),
                None => return Err(ReservationValueError::MissingCardDetails),
            },
            PaymentMethodEnum::Cash => PaymentOption::Cash,
        };
        let special_requests = request.special_requests.try_into()?;

        Ok(ValidatedReservationRequest::new(
            user_id,
            received_timestamp,
            restaurant_id,
            reservation_type,
            payment_option,
            special_requests,
        ))
    }
}
