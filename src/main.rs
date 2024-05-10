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
    mod mapper;
    mod model;
    mod routes;

    pub use model::Receipt;
    pub use model::ReceiptView;
    pub use routes::__path_create;
    pub use routes::__path_delete;
    pub use routes::__path_get_all;
    pub use routes::__path_get_one;
    pub use routes::init_routes;
}

pub mod receipt_item {
    mod mapper;
    mod model;
    mod routes;

    pub use model::ReceiptItem;
    pub use routes::init_routes;
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
