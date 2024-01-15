use actix_web::{delete, get, patch, post, web, HttpResponse};

#[get("/{workflow}/{action}")]
pub async fn get(info: web::Path<(String, String)>) -> HttpResponse {
    let path_data = info.into_inner();
    HttpResponse::Ok().json(path_data)
}

#[patch("/{workflow}/{action}")]
pub async fn patch(info: web::Path<(String, String)>) -> HttpResponse {
    let path_data = info.into_inner();
    HttpResponse::Ok().json(path_data)
}

#[post("/{workflow}/{action}")]
pub async fn post(info: web::Path<(String, String)>) -> HttpResponse {
    let path_data = info.into_inner();
    HttpResponse::Ok().json(path_data)
}

#[delete("/{workflow}/{action}")]
pub async fn delete(info: web::Path<(String, String)>) -> HttpResponse {
    let path_data = info.into_inner();
    HttpResponse::Ok().json(path_data)
}
