use actix_web::{
    HttpResponse, Responder, get,
    web::{self, ServiceConfig},
};
use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static"]
struct StaticFiles;

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(statics);
}

#[get("/static/{path:.*}")]
async fn statics(path: web::Path<String>) -> impl Responder {
    match StaticFiles::get(path.as_str()) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path.to_string()).first_or_octet_stream().as_ref())
            .append_header(("Service-Worker-Allowed", "/"))
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("File not found"),
    }
}
