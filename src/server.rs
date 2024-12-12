use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post, put},
    Extension, Json, Router,
};
use kore_bridge::{model::BridgeSignedEventRequest, Bridge, GovsData, RegisterData, RequestData};
use serde::Deserialize;
use serde_json::Value;
use tower::ServiceBuilder;

use crate::error::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct SubjectQuery {
    active: Option<bool>,
    schema: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GovQuery {
    active: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventsQuery {
    quantity: Option<u64>,
    page: Option<u64>,
}

async fn send_event_request(
    Extension(bridge): Extension<Arc<Bridge>>,
    Json(request): Json<BridgeSignedEventRequest>,
) -> Result<Json<RequestData>, Error> {
    match bridge.send_event_request(request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

async fn get_request_state(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(request_id): Path<String>,
) -> Result<Json<String>, Error> {
    match bridge.get_request_state(request_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

async fn get_approval(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<Value>, Error> {
    match bridge.get_approval(subject_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

async fn patch_approval(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
    Json(response): Json<String>,
) -> Result<Json<String>, Error> {
    match bridge.patch_approve(subject_id, response).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

async fn put_auth(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
    Json(witnesses): Json<Vec<String>>,
) -> Result<Json<String>, Error> {
    match bridge.put_auth_subject(subject_id, witnesses).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

async fn get_all_auth_subjects(
    Extension(bridge): Extension<Arc<Bridge>>,
) -> Result<Json<Vec<String>>, Error> {
    match bridge.get_all_auth_subjects().await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

async fn get_witnesses_subject(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<Vec<String>>, Error> {
    match bridge.get_witnesses_subject(subject_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

async fn delete_auth_subject(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<String>, Error> {
    match bridge.delete_auth_subject(subject_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

async fn update_subject(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<String>, Error> {
    match bridge.update_subject(subject_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

async fn get_all_govs(
    Extension(bridge): Extension<Arc<Bridge>>,
    Query(parameters): Query<GovQuery>,
) -> Result<Json<Vec<GovsData>>, Error> {
    match bridge.get_all_govs(parameters.active).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

async fn get_all_subjects(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(governance_id): Path<String>,
    Query(parameters): Query<SubjectQuery>,
) -> Result<Json<Vec<RegisterData>>, Error> {
    match bridge
        .get_all_subjs(governance_id, parameters.active, parameters.schema)
        .await
    {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

async fn get_events(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
    Query(parameters): Query<EventsQuery>,
) -> Result<Json<Value>, Error> {
    match bridge
        .get_events(subject_id, parameters.quantity, parameters.page)
        .await
    {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

async fn get_state(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<Value>, Error> {
    match bridge.get_subject(subject_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

async fn get_signatures(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<Value>, Error> {
    match bridge.get_signatures(subject_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

async fn get_controller_id(Extension(bridge): Extension<Arc<Bridge>>) -> Json<String> {
    Json(bridge.controller_id())
}

async fn get_peer_id(Extension(bridge): Extension<Arc<Bridge>>) -> Json<String> {
    Json(bridge.peer_id())
}

pub fn build_routes(bridge: Bridge) -> Router {
    let bridge = Arc::new(bridge);
    // TODO añadir ruta para consultar todas las request
    Router::new()
        .route("/signatures/:subject_id", get(get_signatures))
        .route("/state/:subject_id", get(get_state))
        .route("/events/:subject_id", get(get_events))
        .route("/register-subjects/:governance_id", get(get_all_subjects))
        .route("/register-governances", get(get_all_govs))
        .route("/update/:subject_id", post(update_subject))
        .route("/auth/:subject_id", delete(delete_auth_subject))
        .route("/auth/:subject_id", get(get_witnesses_subject))
        .route("/auth", get(get_all_auth_subjects))
        .route("/auth/:subject_id", put(put_auth))
        .route("/approval-request/:subject_id", patch(patch_approval))
        .route("/approval-request/:subject_id", get(get_approval))
        .route("/event-request/:request_id", get(get_request_state))
        .route("/event-request", post(send_event_request))
        .route("/controller-id", get(get_controller_id))
        .route("/peer-id", get(get_peer_id))
        .layer(ServiceBuilder::new().layer(Extension(bridge)))
}
