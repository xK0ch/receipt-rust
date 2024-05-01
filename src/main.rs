#[macro_use]
extern crate diesel_migrations;

use crate::db::establish_connection;

mod api_error;
mod db;
mod receipt;
mod schema;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::info;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("Connecting to database");

    let connection = &mut establish_connection();

    info!("Starting database migration");

    db::run_migration(connection);

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    let server = HttpServer::new(|| App::new().configure(receipt::init_routes))
        .bind(format!("{}:{}", host, port))?;

    info!("Starting server");

    server.run().await
}
