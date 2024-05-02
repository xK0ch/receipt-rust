use crate::api_error::ApiError;
use crate::db::establish_connection;
use crate::schema::receipt;
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{NaiveDateTime, Utc};
use diesel::expression_methods::ExpressionMethods;
use diesel::{Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = crate::schema::receipt)]
pub struct Receipt {
    pub id: Uuid,
    pub sum: BigDecimal,
    pub created_at: NaiveDateTime,
    pub last_modified_at: NaiveDateTime,
}

impl Receipt {
    pub fn get_all() -> Result<Vec<Self>, ApiError> {
        let connection = &mut establish_connection();

        let receipts = receipt::table.load::<Receipt>(connection)?;

        Ok(receipts)
    }

    pub fn get_one(id: Uuid) -> Result<Self, ApiError> {
        let connection = &mut establish_connection();

        let receipt = receipt::table
            .filter(receipt::id.eq(id))
            .first(connection)?;

        Ok(receipt)
    }

    pub fn create() -> Result<Self, ApiError> {
        let connection = &mut establish_connection();

        let receipt_to_be_created = Receipt {
            id: Uuid::new_v4(),
            sum: BigDecimal::from_f64(0.0).unwrap(),
            created_at: Utc::now().naive_utc(),
            last_modified_at: Utc::now().naive_utc(),
        };

        let created_receipt = diesel::insert_into(receipt::table)
            .values(receipt_to_be_created)
            .get_result(connection)?;

        Ok(created_receipt)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let connection = &mut establish_connection();

        let result =
            diesel::delete(receipt::table.filter(receipt::id.eq(id))).execute(connection)?;

        Ok(result)
    }
}
