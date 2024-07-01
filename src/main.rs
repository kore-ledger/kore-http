mod common;
mod config;
mod error;
mod middleware;
mod server;

#[cfg(feature = "doc")]
mod doc;

use std::net::SocketAddr;

use axum::http::{header, Method};
use clap::Parser;
use config::env::build_address;
use kore_node::{
    clap,
    config::{
        build::{build_config, build_file_path, build_password},
        command::Args,
    }
};
#[cfg(feature = "leveldb")]
use kore_node::{KoreNode, LevelDBNode};

#[cfg(feature = "sqlite")]
use kore_node::{KoreNode, SqliteNode};

use middleware::middlewares::tower_trace;
use server::build_routes;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use tracing::debug;
use tracing_subscriber::EnvFilter;
const TARGET_WORKER: &str = "HTTP";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .try_init().unwrap();

    debug!(
        TARGET_WORKER,
        "To the moon!"
    );

    // Command line args.
    let args = Args::parse();

    let mut password = args.password;
    if password.is_empty() {
        password = build_password();
    }

    let mut file_path = args.file_path;
    if file_path.is_empty() {
        file_path = build_file_path();
    }

    let listener = tokio::net::TcpListener::bind(build_address())
        .await
        .unwrap();

    // Settings.
    let kore_settings = build_config(args.env_config, &file_path);
    // Node.
    #[cfg(feature = "leveldb")]
    let node = LevelDBNode::build(kore_settings, &password).unwrap();
    #[cfg(feature = "sqlite")]
    let node = SqliteNode::build(kore_settings, &password).unwrap();
    
    node.bind_with_shutdown(signal::ctrl_c());

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH])
        .allow_headers([header::CONTENT_TYPE])
        .allow_origin(Any);

    let api = node.api().clone();
    axum::serve(
        listener,
        tower_trace(build_routes(api))
            .layer(cors)
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(async move {
        tokio::select! {
            _ = node.token().cancelled() => {
                log::debug!("Shutdown received");
            }
        }
    })
    .await
    .unwrap()
}
