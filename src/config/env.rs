use std::env;

pub fn build_address() -> String {
    env::var("KORE_HTTP_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned())
}