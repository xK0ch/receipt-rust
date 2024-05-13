use rust_decimal::Decimal;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct ReceiptItemUpdateOrder {
    pub name: String,
    pub amount: i32,
    pub price: Decimal,
}
