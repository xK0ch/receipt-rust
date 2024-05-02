use crate::api_error::ApiError;
use crate::receipt_item::model::ReceiptItemCreateOrder;
use crate::receipt_item::ReceiptItem;
use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;

#[get("/receiptItems/{id}")]
async fn get_all_by_receipt(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let receipt = ReceiptItem::get_all_by_receipt(id.into_inner())?;
    Ok(HttpResponse::Ok().json(receipt))
}

#[post("/receiptItems")]
async fn create(
    receipt_item_create_order: web::Json<ReceiptItemCreateOrder>,
) -> Result<HttpResponse, ApiError> {
    let receipt = ReceiptItem::create(receipt_item_create_order.into_inner())?;
    Ok(HttpResponse::Created().json(receipt))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_by_receipt);
    cfg.service(create);
}
