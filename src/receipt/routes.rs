use crate::api_error::ApiError;
use crate::receipt::Receipt;
use actix_web::{delete, get, post, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/receipts")]
async fn get_all() -> Result<HttpResponse, ApiError> {
    let receipts = Receipt::get_all()?;
    Ok(HttpResponse::Ok().json(receipts))
}

#[get("/receipts/{id}")]
async fn get_one(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let receipt = Receipt::get_one(id.into_inner())?;
    Ok(HttpResponse::Ok().json(receipt))
}

#[post("/receipts")]
async fn create() -> Result<HttpResponse, ApiError> {
    let receipt = Receipt::create()?;
    Ok(HttpResponse::Created().json(receipt))
}

#[delete("/receipts/{id}")]
async fn delete(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let num_deleted = Receipt::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all);
    cfg.service(get_one);
    cfg.service(create);
    cfg.service(delete);
}
