use thiserror::Error;
use std::convert::From;

use crate::reservation::data_models::api_schemas::ReservationRequest;
use crate::reservation::data_models::entities::ValidatedReservationRequest;
use crate::reservation::data_models::values::ReservationValueError;


// pub fn accept_reservation()

/* action results */
// happy path
struct VerifiedReservationRequest(ValidatedReservationRequest);
impl From<ValidatedReservationRequest> for VerifiedReservationRequest {
  fn from(item: ValidatedReservationRequest) -> Self {
      VerifiedReservationRequest(item)
  }
}


// error path
#[derive(Error, Debug)]
pub enum ReservationMasterDataError {
    #[error("Restaurant with ID '{0}' not found.")]
    RestaurantNotFoundError(String),

    #[error("Inconsistent master data for restaurant with ID '{0}'.")]
    MasterDataInconsistencyError(String),
}

pub fn validate_request_fields(
  reservation: ReservationRequest
) -> Result<ValidatedReservationRequest, ReservationValueError> {
  reservation.try_into()
}

pub fn verify_request_against_masterdata(
  reservation: ValidatedReservationRequest
) -> Result<VerifiedReservationRequest, ReservationMasterDataError> {
  //TODO: biz logic - match with master data
  Ok(reservation.into())
}
