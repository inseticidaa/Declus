#[macro_use]
extern crate diesel;

use actix_rt;
use dotenv::dotenv;
use actix_web::{App, HttpServer};
use actix::prelude::{SyncArbiter};
use app::create_schema_with_context;

mod app;
mod database;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_pool = database::new_pool().expect("Can't create database pool.");
    let database_address = SyncArbiter::start(2, move || database::DbExecutor(database_pool.clone()));
    let schema = create_schema_with_context(database_address.clone());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .configure(app::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
