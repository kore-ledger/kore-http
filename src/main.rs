mod common;
mod error;
mod middleware;
mod server;
mod util;
mod config;

use std::net::SocketAddr;

use clap::Parser;
use config::env::build_address;
use kore_node::{clap, config::{build::{build_config, build_password}, command::Args}, KoreNode, SqliteNode};
use middleware::middlewares::tower_trace;
use server::build_routes;
use util::logger::build_logger;

#[tokio::main]
async fn main() {
    // Logs
    build_logger();

    // Command line args.
    let args = Args::parse();
    let mut password = args.password;

    if password.is_empty() {
        password = build_password();
    }
    
    let listener = tokio::net::TcpListener::bind(build_address())
        .await
        .unwrap();

    // Settings.
    let kore_settings = build_config(args.env_config, &args.file_path);
    // Node.
    let node = SqliteNode::build(kore_settings, &password).unwrap();
    axum::serve(
        listener,
        tower_trace(build_routes(node.api().clone()))
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
