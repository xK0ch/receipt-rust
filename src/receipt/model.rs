use crate::api_error::ApiError;
use crate::db::establish_connection;
use crate::receipt_item::ReceiptItem;
use crate::schema::receipt;
use crate::schema::receipt::{last_modified_at, sum};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ReceiptView {
    pub id: Uuid,
    pub sum: BigDecimal,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = crate::schema::receipt)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Receipt {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modified_at: DateTime<Utc>,
    pub sum: BigDecimal,
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
            created_at: Utc::now(),
            last_modified_at: Utc::now(),
            sum: BigDecimal::from_str("0.00").unwrap().with_scale(2),
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

    pub fn calculate_sum(receipt_id: Uuid) -> Result<Self, ApiError> {
        let receipt_items = ReceiptItem::get_all_by_receipt(receipt_id).unwrap();
        let initial_sum = BigDecimal::from_str("0.00").unwrap().with_scale(2);
        let receipt_sum =
            receipt_items
                .into_iter()
                .fold(initial_sum, |accumulator, receipt_item| {
                    accumulator.add(receipt_item.price.mul(receipt_item.amount))
                });

        let connection = &mut establish_connection();

        let updated_receipt = diesel::update(receipt::table)
            .filter(receipt::id.eq(receipt_id))
            .set((sum.eq(&receipt_sum), last_modified_at.eq(Utc::now())))
            .get_result(connection)?;

        Ok(updated_receipt)
    }
}
