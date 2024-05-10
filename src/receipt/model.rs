use crate::core::api::api_error::not_found;
use crate::core::database::establish_connection;
use crate::core::database::schema::receipt;
use crate::core::database::schema::receipt::{last_modified_at, sum};
use crate::core::ApiError;
use crate::receipt_item::ReceiptItem;
use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::ops::{Add, Mul};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ReceiptView {
    pub id: Uuid,
    pub sum: Decimal,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable, Clone)]
#[diesel(table_name = crate::core::database::schema::receipt)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Receipt {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modified_at: DateTime<Utc>,
    pub sum: Decimal,
}

impl Receipt {
    pub fn get_all() -> Result<Vec<Self>, ApiError> {
        let receipts = receipt::table.load::<Receipt>(&mut establish_connection())?;

        Ok(receipts)
    }

    pub fn get_one(receipt_id: Uuid) -> Result<Self, ApiError> {
        match receipt::table
            .filter(receipt::id.eq(receipt_id))
            .first(&mut establish_connection())
        {
            Ok(receipt) => Ok(receipt),
            Err(error) => not_found::<Self>(error, "Receipt", receipt_id),
        }
    }

    pub fn create() -> Result<Self, ApiError> {
        let receipt_to_be_created = Receipt {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            last_modified_at: Utc::now(),
            sum: dec!(0.00),
        };

        let created_receipt = diesel::insert_into(receipt::table)
            .values(receipt_to_be_created)
            .get_result(&mut establish_connection())?;

        Ok(created_receipt)
    }

    pub fn delete(receipt: Receipt) -> Result<usize, ApiError> {
        let result = diesel::delete(&receipt).execute(&mut establish_connection())?;

        Ok(result)
    }

    pub fn calculate_sum(receipt_id: Uuid) -> Result<Self, ApiError> {
        let receipt_items = ReceiptItem::get_all_by_receipt(receipt_id).unwrap();
        let initial_sum = dec!(0.00);
        let receipt_sum =
            receipt_items
                .into_iter()
                .fold(initial_sum, |accumulator, receipt_item| {
                    accumulator.add(receipt_item.price.mul(Decimal::from(receipt_item.amount)))
                });

        let connection = &mut establish_connection();

        let updated_receipt = diesel::update(receipt::table)
            .filter(receipt::id.eq(receipt_id))
            .set((sum.eq(&receipt_sum), last_modified_at.eq(Utc::now())))
            .get_result(connection)?;

        Ok(updated_receipt)
    }
}
