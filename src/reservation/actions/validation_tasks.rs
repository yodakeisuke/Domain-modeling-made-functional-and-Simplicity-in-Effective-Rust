use crate::reservation::data_models::api_schemas::ReservationRequest;
use crate::reservation::data_models::entities::VerifiedReservationRequest;
use crate::reservation::data_models::values::ReservationValueError;

pub fn accept_reservation(
  reservation: ReservationRequest
) -> Result<VerifiedReservationRequest, ReservationValueError> {
  reservation.try_into()
}
