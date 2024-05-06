use crate::api_error::ApiError;
use crate::db::establish_connection;
use crate::receipt::Receipt;
use crate::receipt_item::mapper;
use crate::schema::receipt_item;
use crate::schema::receipt_item::{amount, last_modified_at, name, price};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::{
    Associations, BelongingToDsl, ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable,
    RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ReceiptItemView {
    pub id: Uuid,
    pub name: String,
    pub amount: i32,
    pub price: BigDecimal,
}

#[derive(Deserialize)]
pub struct ReceiptItemCreateOrder {
    pub name: String,
    pub amount: i32,
    pub price: BigDecimal,
    pub receipt_id: Uuid,
}

#[derive(Deserialize)]
pub struct ReceiptItemUpdateOrder {
    pub name: String,
    pub amount: i32,
    pub price: BigDecimal,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable)]
#[diesel(belongs_to(Receipt))]
#[diesel(table_name = receipt_item)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReceiptItem {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modified_at: DateTime<Utc>,
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

    pub fn get_one(receipt_item_id: Uuid) -> Result<Self, ApiError> {
        let connection = &mut establish_connection();

        let receipt_item = receipt_item::table
            .filter(receipt_item::id.eq(receipt_item_id))
            .first(connection)?;

        Ok(receipt_item)
    }

    pub fn create(create_order: ReceiptItemCreateOrder) -> Result<Self, ApiError> {
        let connection = &mut establish_connection();

        let receipt = Receipt::get_one(create_order.receipt_id)?;

        let receipt_item_to_be_created = mapper::from_create_order(create_order, receipt.id);

        let created_receipt_item: ReceiptItem = diesel::insert_into(receipt_item::table)
            .values(receipt_item_to_be_created)
            .get_result(connection)?;

        Receipt::calculate_sum(created_receipt_item.receipt_id)?;

        Ok(created_receipt_item)
    }

    pub fn update(
        receipt_item_id: Uuid,
        update_order: ReceiptItemUpdateOrder,
    ) -> Result<Self, ApiError> {
        let connection = &mut establish_connection();

        let updated_receipt_item: ReceiptItem = diesel::update(receipt_item::table)
            .filter(receipt_item::id.eq(receipt_item_id))
            .set((
                name.eq(update_order.name),
                amount.eq(update_order.amount),
                price.eq(update_order.price),
                last_modified_at.eq(Utc::now()),
            ))
            .get_result(connection)?;

        Receipt::calculate_sum(updated_receipt_item.receipt_id)?;

        Ok(updated_receipt_item)
    }

    pub fn delete(receipt_item_id: Uuid) -> Result<usize, ApiError> {
        let connection = &mut establish_connection();

        let receipt_item: ReceiptItem = ReceiptItem::get_one(receipt_item_id)?;

        let result = diesel::delete(&receipt_item).execute(connection)?;

        Receipt::calculate_sum(receipt_item.receipt_id)?;

        Ok(result)
    }
}
