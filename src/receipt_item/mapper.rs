use crate::receipt_item::model::{ReceiptItemCreateOrder, ReceiptItemView};
use crate::receipt_item::ReceiptItem;
use chrono::Utc;
use uuid::Uuid;

pub fn to_view(receipt_item: ReceiptItem) -> ReceiptItemView {
    ReceiptItemView {
        id: receipt_item.id,
        name: receipt_item.name,
        amount: receipt_item.amount,
        price: receipt_item.price,
    }
}

pub fn from_create_order(
    create_order: ReceiptItemCreateOrder,
    receipt_item_id: Uuid,
) -> ReceiptItem {
    ReceiptItem {
        id: Uuid::new_v4(),
        price: create_order.price.with_scale(2),
        name: create_order.name,
        amount: create_order.amount,
        created_at: Utc::now().naive_utc(),
        last_modified_at: Utc::now().naive_utc(),
        receipt_id: receipt_item_id,
    }
}
