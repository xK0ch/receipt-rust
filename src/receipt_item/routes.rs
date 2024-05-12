use crate::core::ApiError;
use crate::receipt_item::model::{ReceiptItemCreateOrder, ReceiptItemUpdateOrder, ReceiptItemView};
use crate::receipt_item::ReceiptItem;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = 200, description = "If the receiptItems were loaded successfully", body = Vec<ReceiptItemView>),
        (status = 404, description = "If the receipt was not found for the given id", body = ApiError)
    ),
    tag = "ReceiptItem"
)]
#[get("/receiptItems/receipts/{receiptId}")]
async fn get_all_receipt_items_by_receipt(
    receipt_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let receipt_items = ReceiptItem::get_all_by_receipt(receipt_id.into_inner())?;

    let receipt_item_views: Vec<ReceiptItemView> = receipt_items
        .into_iter()
        .map(|receipt_item| ReceiptItemView::from(receipt_item))
        .collect();

    Ok(HttpResponse::Ok().json(receipt_item_views))
}

#[utoipa::path(
    responses(
        (status = 200, description = "If the receipt item was loaded successfully", body = ReceiptItemView),
        (status = 404, description = "If the receipt item was not found for the given id", body = ApiError)
    ),
    tag = "ReceiptItem"
)]
#[get("/receiptItems/{receiptItemId}")]
async fn get_one_receipt_item(receipt_item_id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let receipt_item = ReceiptItem::get_one(receipt_item_id.into_inner())?;

    Ok(HttpResponse::Ok().json(ReceiptItemView::from(receipt_item)))
}

#[utoipa::path(
    request_body = ReceiptItemCreateOrder,
    responses(
        (status = 200, description = "If the receiptItem was loaded successfully", body = ReceiptItemView),
        (status = 404, description = "If the receipt was not found for the given id", body = ApiError)
    ),
    tag = "ReceiptItem"
)]
#[post("/receiptItems")]
async fn create_receipt_item(
    create_order: web::Json<ReceiptItemCreateOrder>,
) -> Result<HttpResponse, ApiError> {
    let receipt_item = ReceiptItem::create(create_order.into_inner())?;

    Ok(HttpResponse::Created().json(ReceiptItemView::from(receipt_item)))
}

#[utoipa::path(
    request_body = ReceiptItemUpdateOrder,
    responses(
        (status = 200, description = "If the receipt item was updated successfully", body = ReceiptItemView),
        (status = 404, description = "If the receipt item was not found for the given id", body = ApiError)
    ),
    tag = "ReceiptItem"
)]
#[put("/receiptItems/{receiptItemId}")]
async fn update_receipt_item(
    receipt_item_id: web::Path<Uuid>,
    receipt_item_create_order: web::Json<ReceiptItemUpdateOrder>,
) -> Result<HttpResponse, ApiError> {
    let receipt_item = ReceiptItem::get_one(receipt_item_id.into_inner())?;

    let updated_receipt_item =
        ReceiptItem::update(receipt_item, receipt_item_create_order.into_inner())?;

    Ok(HttpResponse::Ok().json(ReceiptItemView::from(updated_receipt_item)))
}

#[utoipa::path(
    responses(
        (status = 204, description = "If the receipt item was deleted successfully"),
        (status = 404, description = "If the receipt item was not found for the given id", body = ApiError)
    ),
    tag = "ReceiptItem",
)]
#[delete("/receiptItems/{receiptItemId}")]
async fn delete_receipt_item(receipt_item_id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let receipt_item = ReceiptItem::get_one(receipt_item_id.into_inner())?;

    ReceiptItem::delete(receipt_item)?;

    Ok(HttpResponse::NoContent().json(json!({})))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_receipt_items_by_receipt);
    cfg.service(get_one_receipt_item);
    cfg.service(create_receipt_item);
    cfg.service(update_receipt_item);
    cfg.service(delete_receipt_item);
}
