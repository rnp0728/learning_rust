use actix_web::web;

use super::handlers;

pub fn jobs_hub_routes() -> actix_web::Scope {
    web::scope("/jobshub")
        .service(handlers::get_jobs)
        .service(handlers::get_job_by_id)
        .service(handlers::create_job)
        .service(handlers::update_job_by_id)
}
