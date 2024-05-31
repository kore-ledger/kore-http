use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use axum::{
    body::Bytes,
    extract::{MatchedPath, Request},
    http::HeaderMap,
    middleware::Next,
    response::Response,
    Router,
};
use log::{debug, error};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};

use crate::{
    common::{ConnectionsDuration, IPSMaxConnectState},
    error::Errors,
};

// Middleware that is responsible for limiting the number of times a IP can use the API,
// by default it is 500 times in a minute, this middleware prevents DDOS attacks.
pub async fn limit_ip_request(request: Request, next: Next) -> Result<Response, Errors> {
    debug!("Accessing the state");
    let state = match request.extensions().get() {
        Some(state) => {
            debug!("Successfully accessed the state");
            let state: Arc<Mutex<IPSMaxConnectState>> = Arc::clone(state);
            state
        }
        None => {
            error!("Failed to access the state");
            return Err(Errors::State);
        }
    };

    let remote_addr: String = match request
        .extensions()
        .get::<axum::extract::ConnectInfo<SocketAddr>>()
        .map(|ci| ci.0)
    {
        Some(addr) => addr
            .to_string()
            .split(':')
            .next()
            .unwrap_or_default()
            .to_owned(),
        None => {
            return Err(Errors::InvalidIP);
        }
    };

    debug!("Checking ip request attempts");
    let ip_connec = match state.lock().unwrap().ips_connects.get(&remote_addr) {
        Some(old_ip_connec) => {
            let mut new_ip_connec = ConnectionsDuration {
                time: SystemTime::now(),
                connections: 0,
            };

            if new_ip_connec
                .time
                .duration_since(old_ip_connec.time)
                .expect("Clock may have gone backwards")
                .as_secs()
                > 60
            {
                debug!("The time limit has passed, resetting ip request attempts");
                new_ip_connec.connections = 1;
            } else if old_ip_connec.connections < 500 {
                debug!(
                    "The ip has been used fewer attempts than allowed, adding 1 to the usage count"
                );
                new_ip_connec.connections = old_ip_connec.connections + 1;
                new_ip_connec.time = old_ip_connec.time;
            } else {
                debug!("The ip request has been used too many attempts, blocking connection");
                return Err(Errors::MaxAttemptsOfRequests);
            }
            new_ip_connec
        }
        None => {
            debug!("The ip request has never been tried, registering");
            ConnectionsDuration {
                time: SystemTime::now(),
                connections: 1,
            }
        }
    };

    debug!("Updating the state of the ip request");
    state
        .lock()
        .unwrap()
        .ips_connects
        .insert(remote_addr, ip_connec);

    Ok(next.run(request).await)
}

// Tower trace metrics.
pub fn tower_trace(routes: Router) -> Router {
    routes.layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            })
            .on_request(|request: &Request<_>, _span: &Span| {
                debug!("New request: {} {}", request.method(), request.uri().path())
            })
            .on_response(|_response: &Response, latency: Duration, _span: &Span| {
                debug!("Response generated in {:?}", latency)
            })
            .on_body_chunk(|chunk: &Bytes, _latency: Duration, _span: &Span| {
                debug!("Sending {} bytes", chunk.len())
            })
            .on_eos(
                |_trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
                    debug!("Stream closed after {:?}", stream_duration)
                },
            )
            .on_failure(
                |error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                    error!(
                        "Something went wrong {} in {:?}",
                        error.to_string(),
                        latency
                    )
                },
            ),
    )
}
