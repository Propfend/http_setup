use std::net::SocketAddr;

use actix_web::{App, HttpServer};

pub mod http_api;

pub async fn run_server(addr: SocketAddr) -> anyhow::Result<()> {
    HttpServer::new(move || App::new()
        .configure(http_api::home::register)
        .configure(http_api::statics::register)
        .configure(http_api::favicon::register)
        .configure(http_api::manifest::register))
        .bind(addr)?
        .run()
        .await?;

    Ok(())
}
