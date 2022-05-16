use actix_web::{Scope, web};

pub mod user_controller;

pub fn v1_scope() -> Scope {
    web::scope("/v1")
        .service(user_controller::user_scope())
}