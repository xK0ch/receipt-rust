use crate::api_error::ApiError;
use crate::receipt_item::model::{ReceiptItemCreateOrder, ReceiptItemUpdateOrder, ReceiptItemView};
use crate::receipt_item::{mapper, ReceiptItem};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/receiptItems/receipts/{receiptId}")]
async fn get_all_by_receipt(receipt_id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let receipt_items = ReceiptItem::get_all_by_receipt(receipt_id.into_inner())?;
    let receipt_item_views: Vec<ReceiptItemView> = receipt_items
        .into_iter()
        .map(|receipt_item| mapper::to_view(receipt_item))
        .collect();
    Ok(HttpResponse::Ok().json(receipt_item_views))
}

#[get("/receiptItems/{receiptItemId}")]
async fn get_one(receipt_item_id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let receipt_item = ReceiptItem::get_one(receipt_item_id.into_inner())?;
    Ok(HttpResponse::Ok().json(mapper::to_view(receipt_item)))
}

#[post("/receiptItems")]
async fn create(
    receipt_item_create_order: web::Json<ReceiptItemCreateOrder>,
) -> Result<HttpResponse, ApiError> {
    let receipt_item = ReceiptItem::create(receipt_item_create_order.into_inner())?;
    Ok(HttpResponse::Created().json(mapper::to_view(receipt_item)))
}

#[put("/receiptItems/{receiptItemId}")]
async fn update(
    receipt_item_id: web::Path<Uuid>,
    receipt_item_create_order: web::Json<ReceiptItemUpdateOrder>,
) -> Result<HttpResponse, ApiError> {
    let receipt_item = ReceiptItem::update(
        receipt_item_id.into_inner(),
        receipt_item_create_order.into_inner(),
    )?;
    Ok(HttpResponse::Created().json(mapper::to_view(receipt_item)))
}

#[delete("/receiptItems/{receiptItemId}")]
async fn delete(receipt_item_id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    ReceiptItem::delete(receipt_item_id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({})))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_by_receipt);
    cfg.service(get_one);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
