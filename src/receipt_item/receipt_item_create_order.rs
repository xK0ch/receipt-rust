use rust_decimal::Decimal;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct ReceiptItemCreateOrder {
    pub name: String,
    pub amount: i32,
    pub price: Decimal,
    pub receipt_id: Uuid,
}
