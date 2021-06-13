#[macro_use]
extern crate diesel;

use actix::prelude::SyncArbiter;
use actix_rt;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use app::create_schema_with_context;
use dotenv::dotenv;
use env_logger::Env;
use num_cpus;

mod app;
mod database;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let database_pool = database::new_pool().expect("Can't create database pool.");
    let database_address = SyncArbiter::start(num_cpus::get(), move || {
        database::DbExecutor(database_pool.clone())
    });
    let schema = create_schema_with_context(database_address.clone());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .configure(app::config)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
