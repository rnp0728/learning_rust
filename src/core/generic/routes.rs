use crate::core::generic::handler::{delete, get, patch, post};
use actix_web::web;

pub fn generic_routes() -> actix_web::Scope {
    web::scope("/generic")
        .service(get)
        .service(patch)
        .service(post)
        .service(delete)
}
