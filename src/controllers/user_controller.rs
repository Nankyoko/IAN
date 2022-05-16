use actix_web::{get, post, web, HttpResponse, error::BlockingError, Scope};
use diesel::prelude::*;

use crate::{DbPool, dberror::DbError, models::{User, CreateUser, insert_new_user}, schema};

pub fn user_scope() -> Scope {
    web::scope("/user")
        .service(insert_user)
        .service(get_user)
}

#[post("register")]
pub async fn insert_user(pool: web::Data<DbPool>, json: web::Json<CreateUser>) -> Result<HttpResponse, DbError> {
    let new_user = json.into_inner();
    let conn = pool.get().expect("Couldn't get db connection from pool.");

    let user = web::block(move || insert_new_user(&conn, new_user))
        .await?;

    match user {
        Ok(val) => Ok(HttpResponse::Created().json(val)),
        Err(e) => Err(DbError::from(e)),
    }
}

#[get("{name}")]
pub async fn get_user(pool: web::Data<DbPool>, name: web::Path<String>) -> Result<HttpResponse, BlockingError> {
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