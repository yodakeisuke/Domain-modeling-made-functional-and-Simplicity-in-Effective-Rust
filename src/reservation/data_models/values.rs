use std::convert::TryFrom;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use thiserror::Error;
use chrono::ParseError;


#[derive(Error, Debug)]
pub enum ReservationValueError {
    #[error("Invalid value for RestaurantId: Expected 10 characters, got {0}")]
    InvalidRestaurantId(usize),

    #[error("Invalid value for UserId: Expected 12 characters, got {0}")]
    InvalidUserId(usize),

    #[error("Invalid number of guests: Expected 1 to 99, got {0}")]
    InvalidNumberOfGuests(u32),

    #[error("Invalid course name: Expected at most 100 characters, got {0}")]
    InvalidCourseName(usize),

    #[error("Invalid special request: Expected at most 500 characters, got {0}")]
    InvalidSpecialRequest(usize),

    #[error("Parsing error: {0}")]
    ParseError(String),

    #[error("Course specified but Missing Course Name ")]
    MissingCourseName,

    #[error("Card Payment specified but Missing Card Details ")]
    MissingCardDetails,
}

impl From<ParseError> for ReservationValueError {
    fn from(err: ParseError) -> Self {
        ReservationValueError::ParseError(err.to_string())
    }
}

#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq, Clone)]
pub struct RestaurantId(String);

impl TryFrom<String> for RestaurantId {
    type Error = ReservationValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() == 10 {
            Ok(RestaurantId(value))
        } else {
            Err(ReservationValueError::InvalidRestaurantId(value.len()))
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq, Clone)]
pub struct UserId(String);

impl TryFrom<String> for UserId {
    type Error = ReservationValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() == 12 {
            Ok(UserId(value))
        } else {
            Err(ReservationValueError::InvalidUserId(value.len()))
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq, Clone)]
pub struct NumberOfGuests(u32);

impl TryFrom<u32> for NumberOfGuests {
    type Error = ReservationValueError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if (1..=99).contains(&value) {
            Ok(NumberOfGuests(value))
        } else {
            Err(ReservationValueError::InvalidNumberOfGuests(value))
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq, Clone)]
pub struct CourseName(String);

impl TryFrom<String> for CourseName {
    type Error = ReservationValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 100 {
            Ok(CourseName(value))
        } else {
            Err(ReservationValueError::InvalidCourseName(value.len()))
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq, Clone)]
pub struct SpecialRequests(Option<String>);

impl TryFrom<Option<String>> for SpecialRequests {
    type Error = ReservationValueError;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        match value {
            Some(ref text) if text.len() > 500 => Err(ReservationValueError::InvalidSpecialRequest(text.len())),
            _ => Ok(SpecialRequests(value)),
        }
    }
}
