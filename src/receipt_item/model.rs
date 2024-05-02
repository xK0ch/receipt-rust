use crate::api_error::ApiError;
use crate::db::establish_connection;
use crate::receipt::Receipt;
use crate::schema::receipt_item;
use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, Utc};
use diesel::{Associations, BelongingToDsl, Identifiable, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ReceiptItemCreateOrder {
    pub name: String,
    pub amount: i32,
    pub price: BigDecimal,
    pub receipt_id: Uuid,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable)]
#[diesel(belongs_to(Receipt))]
#[diesel(table_name = receipt_item)]
pub struct ReceiptItem {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub last_modified_at: NaiveDateTime,
    pub name: String,
    pub amount: i32,
    pub price: BigDecimal,
    pub receipt_id: Uuid,
}

impl ReceiptItem {
    pub fn get_all_by_receipt(receipt_id: Uuid) -> Result<Vec<Self>, ApiError> {
        let connection = &mut establish_connection();

        let receipt = Receipt::get_one(receipt_id)?;

        let receipt_items = ReceiptItem::belonging_to(&receipt).load(connection)?;

        Ok(receipt_items)
    }

    pub fn create(create_order: ReceiptItemCreateOrder) -> Result<Self, ApiError> {
        let connection = &mut establish_connection();

        let receipt = Receipt::get_one(create_order.receipt_id)?;

        let receipt_item_to_be_created = ReceiptItem {
            id: Uuid::new_v4(),
            price: create_order.price.with_scale(2),
            name: create_order.name,
            amount: create_order.amount,
            created_at: Utc::now().naive_utc(),
            last_modified_at: Utc::now().naive_utc(),
            receipt_id: receipt.id,
        };

        let created_receipt_item = diesel::insert_into(receipt_item::table)
            .values(receipt_item_to_be_created)
            .get_result(connection)?;

        Ok(created_receipt_item)
    }
}
