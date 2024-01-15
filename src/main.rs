use core::generic::routes::generic_routes;
use crate::api::jobs_hub::routes;
use crate::db::mongo::connect_db;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use serde::Serialize;

mod models;
mod db;
mod api {
    pub mod jobs_hub {
        pub mod routes;
        pub mod handlers;
    }
}

mod core {
    pub mod generic {
        pub mod routes;
        pub mod handler;
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: &'static str,
    status_code: &'static str,
    status_message: &'static str,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = connect_db().await.unwrap();
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(logger)
            .app_data(web::JsonConfig::default().limit(4096))
            .configure(app_config)
            .default_service(web::route().to(not_found))
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}

fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/app")
            .service(system_routes)
            .service(routes::jobs_hub_routes())
            .service(generic_routes()),
    );
}

#[actix_web::get("/status")]
async fn system_routes() -> HttpResponse {
    HttpResponse::Ok().json("System status endpoint")
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(ErrorResponse {
        status: "error",
        status_code: "404",
        status_message: "The requested resource was not found!",
    })
}
