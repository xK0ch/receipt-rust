use crate::receipt_item::model::{ReceiptItemCreateOrder, ReceiptItemView};
use crate::receipt_item::ReceiptItem;
use chrono::Utc;
use uuid::Uuid;

impl From<ReceiptItem> for ReceiptItemView {
    fn from(receipt_item: ReceiptItem) -> Self {
        ReceiptItemView {
            id: receipt_item.id,
            name: receipt_item.name,
            amount: receipt_item.amount,
            price: receipt_item.price,
        }
    }
}

impl From<(ReceiptItemCreateOrder, Uuid)> for ReceiptItem {
    fn from((create_order, receipt_id): (ReceiptItemCreateOrder, Uuid)) -> Self {
        ReceiptItem {
            id: Uuid::new_v4(),
            price: create_order.price.with_scale(2),
            name: create_order.name,
            amount: create_order.amount,
            created_at: Utc::now(),
            last_modified_at: Utc::now(),
            receipt_id,
        }
    }
}
