#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
mod frontend;
mod dberror;
mod controllers;

use actix_web::{web, App, HttpServer};
use actix_files as fs;
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use frontend::serve_external_frontend;
use r2d2::PooledConnection;
use controllers::v1_scope;
use std::env;



type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
type PoolConn = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL Enviornment variable must be set.");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to initialize connection pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(serve_external_frontend))
            .service(
                web::scope("/api")
                    .service(v1_scope())
            )
            .service(fs::Files::new("/", "./public").index_file("index.html"))

    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}