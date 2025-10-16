use std::error::Error;

use actix_web::{
    HttpResponse, Responder, get,
    web::ServiceConfig,
};
use askama::Template;
use esbuild_metafile::HttpPreloader;
use esbuild_metafile::filters;

#[derive(Template)]
#[template(path = "index.html")]
struct Home {
    preloads: HttpPreloader,
}

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(home);
}

#[get("/")]
async fn home(preloads: HttpPreloader) -> Result<impl Responder, Box<dyn Error>> {
    let template = Home {
        preloads
    };
    let rendered = template.render()?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}
