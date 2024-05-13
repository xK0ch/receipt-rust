#[macro_use]
extern crate diesel_migrations;

pub mod core {
    pub use api::ApiError;

    pub mod api {
        pub mod api_documentation;
        pub mod api_error;

        pub use api_error::ApiError;
    }
    pub mod database {
        mod db;
        pub mod schema;

        pub use db::establish_connection;
        pub use db::run_migration;
    }
}

pub mod receipt {
    mod controller;
    mod receipt;
    mod receipt_view;

    pub use controller::__path_create_receipt;
    pub use controller::__path_delete_receipt;
    pub use controller::__path_get_all_receipts;
    pub use controller::__path_get_one_receipt;
    pub use controller::init_routes;
    pub use receipt::Receipt;
    pub use receipt_view::ReceiptView;
}

pub mod receipt_item {
    mod controller;
    mod receipt_item;
    mod receipt_item_create_order;
    mod receipt_item_update_order;
    mod receipt_item_view;

    pub use controller::__path_create_receipt_item;
    pub use controller::__path_delete_receipt_item;
    pub use controller::__path_get_all_receipt_items_by_receipt;
    pub use controller::__path_get_one_receipt_item;
    pub use controller::__path_update_receipt_item;
    pub use controller::init_routes;
    pub use receipt_item::ReceiptItem;
    pub use receipt_item_create_order::ReceiptItemCreateOrder;
    pub use receipt_item_update_order::ReceiptItemUpdateOrder;
    pub use receipt_item_view::ReceiptItemView;
}

use crate::core::api::api_documentation::ApiDoc;
use crate::core::database::establish_connection;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::info;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("Generating api documentation");
    let openapi = ApiDoc::openapi();

    info!("Connecting to database");
    let connection = &mut establish_connection();

    info!("Starting database migration");
    core::database::run_migration(connection);

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    let server = HttpServer::new(move || {
        App::new()
            .configure(receipt::init_routes)
            .configure(receipt_item::init_routes)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(format!("{}:{}", host, port))?;

    info!("Starting server");

    server.run().await
}
