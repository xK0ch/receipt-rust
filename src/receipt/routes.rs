use crate::core::ApiError;
use crate::receipt::model::ReceiptView;
use crate::receipt::{mapper, Receipt};
use actix_web::{delete, get, post, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/receipts")]
async fn get_all() -> Result<HttpResponse, ApiError> {
    let receipts = Receipt::get_all()?;
    let receipt_views: Vec<ReceiptView> = receipts
        .into_iter()
        .map(|receipt| mapper::to_view(receipt))
        .collect();
    Ok(HttpResponse::Ok().json(receipt_views))
}

#[get("/receipts/{id}")]
async fn get_one(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let receipt = Receipt::get_one(id.into_inner())?;
    Ok(HttpResponse::Ok().json(mapper::to_view(receipt)))
}

#[post("/receipts")]
async fn create() -> Result<HttpResponse, ApiError> {
    let receipt = Receipt::create()?;
    Ok(HttpResponse::Created().json(mapper::to_view(receipt)))
}

#[delete("/receipts/{id}")]
async fn delete(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    Receipt::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({})))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all);
    cfg.service(get_one);
    cfg.service(create);
    cfg.service(delete);
}
