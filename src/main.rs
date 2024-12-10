use std::net::SocketAddr;

use axum::http::{header, Method};
use enviroment::build_address;
use middleware::tower_trace;
use server::build_routes;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;
use kore_bridge::{clap::Parser, settings::{build_config, build_file_path, build_password, command::Args}, Bridge};

mod server;
mod enviroment;
mod middleware;
mod error;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .try_init().unwrap();

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

    let config = build_config(args.env_config, &file_path);
    let bridge = Bridge::build(config, &password, None).await.unwrap();
    let token = bridge.token().clone();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH])
        .allow_headers([header::CONTENT_TYPE])
        .allow_origin(Any);
    axum::serve(
        listener,
        tower_trace(build_routes(bridge))
            .layer(cors)
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(async move {
        tokio::select! {
            _ = token.cancelled() => {
            }
        }
    })
    .await
    .unwrap()

}
