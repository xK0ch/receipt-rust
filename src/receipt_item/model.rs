use crate::core::api::api_error::not_found;
use crate::core::database::establish_connection;
use crate::core::database::schema::receipt_item;
use crate::core::database::schema::receipt_item::{amount, last_modified_at, name, price};
use crate::core::ApiError;
use crate::receipt::Receipt;
use chrono::{DateTime, Utc};
use diesel::{
    Associations, BelongingToDsl, ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable,
    RunQueryDsl,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct ReceiptItemView {
    pub id: Uuid,
    pub name: String,
    pub amount: i32,
    pub price: Decimal,
}

#[derive(Deserialize, IntoParams)]
pub struct ReceiptItemCreateOrder {
    pub name: String,
    pub amount: i32,
    pub price: Decimal,
    pub receipt_id: Uuid,
}

#[derive(Deserialize, IntoParams)]
pub struct ReceiptItemUpdateOrder {
    pub name: String,
    pub amount: i32,
    pub price: Decimal,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable)]
#[diesel(belongs_to(Receipt))]
#[diesel(table_name = crate::core::database::schema::receipt_item)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReceiptItem {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modified_at: DateTime<Utc>,
    pub name: String,
    pub amount: i32,
    pub price: Decimal,
    pub receipt_id: Uuid,
}

impl ReceiptItem {
    pub fn get_all_by_receipt(receipt_id: Uuid) -> Result<Vec<Self>, ApiError> {
        let receipt = Receipt::get_one(receipt_id)?;

        let receipt_items = ReceiptItem::belonging_to(&receipt).load(&mut establish_connection())?;

        Ok(receipt_items)
    }

    pub fn get_one(receipt_item_id: Uuid) -> Result<Self, ApiError> {
        match receipt_item::table
            .filter(receipt_item::id.eq(receipt_item_id))
            .first(&mut establish_connection())
        {
            Ok(receipt) => Ok(receipt),
            Err(error) => not_found::<Self>(error, "ReceiptItem", receipt_item_id),
        }
    }

    pub fn create(create_order: ReceiptItemCreateOrder) -> Result<Self, ApiError> {
        let receipt = Receipt::get_one(create_order.receipt_id)?;

        let receipt_item_to_be_created: ReceiptItem = (create_order, receipt.id).into();

        let created_receipt_item: ReceiptItem = diesel::insert_into(receipt_item::table)
            .values(receipt_item_to_be_created)
            .get_result(&mut establish_connection())?;

        Receipt::calculate_sum(created_receipt_item.receipt_id)?;

        Ok(created_receipt_item)
    }

    pub fn update(
        receipt_item: ReceiptItem,
        update_order: ReceiptItemUpdateOrder,
    ) -> Result<Self, ApiError> {
        let updated_receipt_item: ReceiptItem = diesel::update(&receipt_item)
            .set((
                name.eq(update_order.name),
                amount.eq(update_order.amount),
                price.eq(update_order.price),
                last_modified_at.eq(Utc::now()),
            ))
            .get_result(&mut establish_connection())?;

        Receipt::calculate_sum(updated_receipt_item.receipt_id)?;

        Ok(updated_receipt_item)
    }

    pub fn delete(receipt_item: ReceiptItem) -> Result<usize, ApiError> {
        let result = diesel::delete(&receipt_item).execute(&mut establish_connection())?;

        Receipt::calculate_sum(receipt_item.receipt_id)?;

        Ok(result)
    }

    pub fn delete_all_by_receipt(receipt: Receipt) -> Result<usize, ApiError> {
        let result = diesel::delete(ReceiptItem::belonging_to(&receipt))
            .execute(&mut establish_connection())?;

        Ok(result)
    }
}
