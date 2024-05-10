use crate::core::ApiError;
use crate::receipt::model::ReceiptView;
use crate::receipt::Receipt;
use crate::receipt_item::ReceiptItem;
use actix_web::{delete, get, post, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[utoipa::path(
responses(
(status = 200, description = "If the receipts where loaded successfully", body = Vec<ReceiptView>)
),
tag = "Receipt"
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
(status = 200, description = "If the receipt was loaded successfully", body = ReceiptView),
(status = 404, description = "If the receipt was not found for the given id", body = ApiError)
),
tag = "Receipt"
)]
#[get("/receipts/{receiptId}")]
async fn get_one(receipt_id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let receipt = Receipt::get_one(receipt_id.into_inner())?;

    Ok(HttpResponse::Ok().json(ReceiptView::from(receipt)))
}

#[utoipa::path(
responses(
(status = 200, description = "If the receipt was created successfully", body = ReceiptView)
),
tag = "Receipt"
)]
#[post("/receipts")]
async fn create() -> Result<HttpResponse, ApiError> {
    let receipt = Receipt::create()?;

    Ok(HttpResponse::Created().json(ReceiptView::from(receipt)))
}

#[utoipa::path(
responses(
(status = 204, description = "If the receipt was deleted successfully"),
(status = 404, description = "If the receipt was not found for the given id", body = ApiError)
),
tag = "Receipt"
)]
#[delete("/receipts/{receiptId}")]
async fn delete(receipt_id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let found_receipt: Receipt = Receipt::get_one(receipt_id.into_inner())?;

    ReceiptItem::delete_all_by_receipt(found_receipt.clone())?;
    Receipt::delete(found_receipt)?;

    Ok(HttpResponse::NoContent().json(json!({})))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all);
    cfg.service(get_one);
    cfg.service(create);
    cfg.service(delete);
}
