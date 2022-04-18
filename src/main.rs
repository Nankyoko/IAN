#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result, error::BlockingError, post, ResponseError};
use diesel::{r2d2::ConnectionManager, PgConnection, prelude::*};
use dotenv::dotenv;
use models::{User, CreateUser};
use r2d2::PooledConnection;
use std::{env, fmt::Display};

use crate::models::insert_new_user;

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
            .service(hello)
            .service(insert_user)
            .service(web::resource("/users/{name}").route(web::get().to(get_user)))

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

//http://localhost/
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("<H1>HELLO WORLD!</H1>")
}

#[derive(Debug)]
struct DbError {
    inner_err: anyhow::Error
}

impl Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner_err.fmt(f)
    }
}

impl From<anyhow::Error> for DbError {
    fn from(err: anyhow::Error) -> Self {
        DbError { inner_err: err }
    }
}

impl From<BlockingError> for DbError {
    fn from(err: BlockingError) -> Self {
        DbError {
            inner_err: anyhow::anyhow!("{}", err.to_string())
        }
    }
}

impl ResponseError for DbError{}

#[post("/user")]
async fn insert_user(pool: web::Data<DbPool>, json: web::Json<CreateUser>) -> Result<HttpResponse, DbError> {
    let new_user = json.into_inner();
    let conn = pool.get().expect("Couldn't get db connection from pool.");

    let user = web::block(move || insert_new_user(&conn, new_user))
        .await?;

    match user {
        Ok(val) => Ok(HttpResponse::Created().json(val)),
        Err(e) => Err(DbError::from(e)),
    }
}

async fn get_user(pool: web::Data<DbPool>, name: web::Path<(String)>) -> Result<HttpResponse, BlockingError> {
    let name = name.into_inner();
    let conn = pool.get().expect("Couldn't get db connection from pool.");

    let user = web::block(move || {
        use schema::users::dsl::*;

        users.filter(username.eq(name))
            .first::<User>(&conn)
            .expect("Error loading user.")
    })
    .await?;

    Ok(HttpResponse::Ok().json(user))
}