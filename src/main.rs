use std::net::SocketAddr;

use clap::{Parser, Subcommand};
use esbuild_metafile::instance::initialize_instance;

use crate::{server::run_server, utils::resolve_socket_addr::parse_socket_addr};

mod server;
mod utils;

pub const ESBUILD_META_CONTENTS: &str = include_str!("../esbuild-meta.json");

#[derive(Parser, Debug)]
pub struct ServerCommand {
    #[command(subcommand)]
    commands: Subcommands,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
    Server(Server),
}

#[derive(Parser, Debug)]
struct Server {
    #[arg(long, value_parser = parse_socket_addr)]
    addr: SocketAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let commands = ServerCommand::parse().commands;

    match commands {
        Subcommands::Server(server) => {
            initialize_instance(ESBUILD_META_CONTENTS);
            run_server(server.addr).await?;
        }
    }

    Ok(())
}
