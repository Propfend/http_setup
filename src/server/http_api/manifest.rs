use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::get;
use actix_web::web;

const MANIFEST: &[u8] = include_bytes!("../../../resources/manifest/manifest.json");

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(respond);
}

#[get("/manifest.json")]
async fn respond() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(MANIFEST)
}
