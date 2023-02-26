use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/library")
            .route(web::get().to(get_library))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

pub async fn get_library() -> HttpResponse {
    HttpResponse::Ok().json("todo")
}
