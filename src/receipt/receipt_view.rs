use crate::receipt::Receipt;
use rust_decimal::Decimal;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct ReceiptView {
    pub id: Uuid,
    pub sum: Decimal,
}

impl From<Receipt> for ReceiptView {
    fn from(receipt: Receipt) -> Self {
        ReceiptView {
            id: receipt.id,
            sum: receipt.sum,
        }
    }
}
