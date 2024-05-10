use crate::core::ApiError;
use crate::receipt::model::ReceiptView;
use crate::receipt::Receipt;
use crate::receipt_item::ReceiptItem;
use actix_web::{delete, get, post, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[utoipa::path(
responses(
(status = 200, description = "OK", body = ReceiptView)
),
)]
#[get("/receipts")]
pub async fn get_all() -> Result<HttpResponse, ApiError> {
    let receipts = Receipt::get_all()?;

    let receipt_views: Vec<ReceiptView> = receipts
        .into_iter()
        .map(|receipt| ReceiptView::from(receipt))
        .collect();

    Ok(HttpResponse::Ok().json(receipt_views))
}

#[utoipa::path(
responses(
(status = 200, description = "OK", body = ReceiptView),
(status = 404, description = "NOT FOUND", body = ApiError)
),
)]
#[get("/receipts/{receiptId}")]
async fn get_one(receipt_id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let receipt = Receipt::get_one(receipt_id.into_inner())?;

    Ok(HttpResponse::Ok().json(ReceiptView::from(receipt)))
}

#[utoipa::path(
responses(
(status = 200, description = "OK", body = ReceiptView),),
)]
#[post("/receipts")]
async fn create() -> Result<HttpResponse, ApiError> {
    let receipt = Receipt::create()?;

    Ok(HttpResponse::Created().json(ReceiptView::from(receipt)))
}

#[utoipa::path(
responses(
(status = 200, description = "OK", body = ReceiptView),),
)]
#[delete("/receipts/{receiptId}")]
async fn delete(receipt_id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let found_receipt: Receipt = Receipt::get_one(receipt_id.into_inner())?;

    ReceiptItem::delete_all_by_receipt(found_receipt.clone())?;
    Receipt::delete(found_receipt.clone())?;

    Ok(HttpResponse::Ok().json(json!({})))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all);
    cfg.service(get_one);
    cfg.service(create);
    cfg.service(delete);
}
