use crate::receipt_item::ReceiptItem;
use rust_decimal::Decimal;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema, PartialEq, Eq, Debug)]
pub struct ReceiptItemView {
    pub id: Uuid,
    pub name: String,
    pub amount: i32,
    pub price: Decimal,
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rust_decimal_macros::dec;

    #[test]
    fn from() {
        let receipt_item = ReceiptItem {
            id: Uuid::new_v4(),
            price: dec!(1.54),
            name: "Test".to_string(),
            amount: 1,
            created_at: Utc::now(),
            last_modified_at: Utc::now(),
            receipt_id: Uuid::new_v4(),
        };

        let expected_result = ReceiptItemView {
            id: receipt_item.clone().id,
            price: receipt_item.clone().price,
            name: receipt_item.clone().name,
            amount: receipt_item.clone().amount,
        };

        let result = ReceiptItemView::from(receipt_item);
        assert_eq!(result, expected_result);
    }
}
