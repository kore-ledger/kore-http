mod common;
mod error;
mod middleware;
mod server;
mod util;

use std::net::SocketAddr;

use kore_node::{config::build::build_config, KoreNode, SqliteNode};
use middleware::middlewares::tower_trace;
use server::build_routes;
use util::logger::build_logger;

#[tokio::main]
async fn main() {
    build_logger();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();

    let kore_settings = build_config(true, "");
    let node = SqliteNode::build(kore_settings, "password").unwrap();
    axum::serve(
        listener,
        tower_trace(build_routes(node.api().clone()))
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
