use sqlx::{FromRow, Row};
use chrono::{DateTime, Utc};


pub async fn get_restaurant_by_id(
    pool: &sqlx::MySqlPool,
    restaurant_id: &str,
) -> Result<Option<RestaurantResource>, sqlx::Error> {
    let restaurant = sqlx::query_as::<_, RestaurantResource>(
        r#"
        SELECT
            rr.id,
            rr.restaurant_id,
            rr.created_at,
            rnm.restaurant_name,
            pmm.supports_card AS card,
            pmm.supports_cash AS cash,
            stm.supports_alacarte AS alacarte,
            stm.supports_course AS course
        FROM restaurant_resource AS rr
        JOIN restaurant_name_master AS rnm ON rr.name_id = rnm.name_id
        JOIN payment_methods_master AS pmm ON rr.payment_method_id = pmm.payment_method_id
        JOIN service_types_master AS stm ON rr.service_type_id = stm.service_type_id
        WHERE rr.restaurant_id = ?
        "#,
    )
    .bind(restaurant_id)
    .fetch_optional(pool)
    .await?;

    Ok(restaurant)
}

#[derive(Debug)]
pub struct RestaurantResource {
    id: i32,
    restaurant_id: String,
    created_at: DateTime<Utc>,
    restaurant_name: String,
    supports_payment_method: SupportsPaymentMethod,
    supports_service_type: SupportsServiceType,
}

#[derive(Debug)]
struct SupportsPaymentMethod {
    card: bool,
    cash: bool,
}

#[derive(Debug)]
struct SupportsServiceType {
    alacarte: bool,
    course: bool,
}

impl FromRow<'_, sqlx::mysql::MySqlRow> for RestaurantResource {
    fn from_row(row: &sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(RestaurantResource {
            id: row.try_get("id")?,
            restaurant_id: row.try_get("restaurant_id")?,
            created_at: row.try_get("created_at")?,
            restaurant_name: row.try_get("restaurant_name")?,
            supports_payment_method: SupportsPaymentMethod {
                card: row.try_get("card")?,
                cash: row.try_get("cash")?,
            },
            supports_service_type: SupportsServiceType {
                alacarte: row.try_get("alacarte")?,
                course: row.try_get("course")?,
            },
        })
    }
}
