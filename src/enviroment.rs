use std::env;

pub fn build_address() -> String {
    env::var("KORE_HTTP_ADDRESS").unwrap_or("0.0.0.0:3000".to_owned())
}
