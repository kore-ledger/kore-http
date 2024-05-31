use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, Query},
    routing::{get, patch, post, put},
    Extension, Json, Router,
};
use kore_node::{model::*, KoreApi};
use log::debug;
use tower::ServiceBuilder;

use crate::{common::IPSMaxConnectState, error::Errors, middleware::middlewares::limit_ip_request};

async fn send_event_request(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Json(request): Json<NodeSignedEventRequest>,
) -> Result<Json<EventRequestResponse>, Errors> {
    match kore_api.send_event_request(request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

async fn get_event_request(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Path(request_id): Path<String>,
) -> Result<Json<NodeSignedEventRequest>, Errors> {
    match kore_api.get_event_request(&request_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

async fn get_event_request_state(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Path(request_id): Path<String>,
) -> Result<Json<NodeKoreRequestState>, Errors> {
    match kore_api.get_event_request_state(&request_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

async fn get_approvals(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Query(parameters): Query<NodeGetApprovals>,
) -> Result<Json<Vec<NodeApprovalEntity>>, Errors> {
    match kore_api.get_approvals(parameters).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

async fn get_approval_id(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Path(id): Path<String>,
) -> Result<Json<NodeApprovalEntity>, Errors> {
    match kore_api.get_approval_id(&id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

async fn approval_request(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Path(id): Path<String>,
    Json(vote): Json<PatchVote>,
) -> Result<Json<NodeApprovalEntity>, Errors> {
    match kore_api.approval_request(&id, vote).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

async fn get_all_allowed_subjects_and_providers(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Query(parameters): Query<PaginatorFromString>,
) -> Result<Json<Vec<PreauthorizedSubjectsResponse>>, Errors> {
    match kore_api
        .get_all_allowed_subjects_and_providers(parameters)
        .await
    {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

async fn add_preauthorize_subject(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Path(subject_id): Path<String>,
    Json(authorize_subject): Json<AuthorizeSubject>,
) -> Result<Json<String>, Errors> {
    match kore_api
        .add_preauthorize_subject(&subject_id, authorize_subject)
        .await
    {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

async fn register_keys(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Query(parameters): Query<NodeKeys>,
) -> Result<Json<String>, Errors> {
    match kore_api.register_keys(parameters).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

async fn get_subjects(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Query(parameters): Query<NodeSubjects>,
) -> Result<Json<Vec<NodeSubjectData>>, Errors> {
    match kore_api.get_subjects(parameters).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

async fn get_subject(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Path(subject_id): Path<String>,
) -> Result<Json<NodeSubjectData>, Errors> {
    match kore_api.get_subject(&subject_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

async fn get_validation_proof(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Path(subject_id): Path<String>,
) -> Result<Json<NodeProof>, Errors> {
    match kore_api.get_validation_proof(&subject_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

async fn get_events_of_subject(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Path(subject_id): Path<String>,
    Query(parameters): Query<PaginatorFromNumber>,
) -> Result<Json<Vec<NodeSigned<EventContentResponse>>>, Errors> {
    match kore_api
        .get_events_of_subject(&subject_id, parameters)
        .await
    {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

async fn get_event_of_subject(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Path(subject_id): Path<String>,
    Path(sn): Path<u64>,
) -> Result<Json<NodeSigned<EventContentResponse>>, Errors> {
    match kore_api.get_event_of_subject(&subject_id, sn).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

pub fn build_routes(kore_api: KoreApi) -> Router {
    debug!("Creating states");
    let ips_connects_state = Arc::new(Mutex::new(IPSMaxConnectState {
        ips_connects: HashMap::default(),
    }));

    let kore_api = Arc::new(kore_api);
    let routes = Router::new()
        .route("/event-requests", post(send_event_request))
        .route("/event-requests/:request_id", get(get_event_request))
        .route(
            "/event-requests/:request_id/state",
            get(get_event_request_state),
        )
        .route("/approval-requests", get(get_approvals))
        .route("/approval-requests/:id", get(get_approval_id))
        .route("/approval-requests/:id", patch(approval_request))
        .route(
            "/allowed-subjects",
            get(get_all_allowed_subjects_and_providers),
        )
        .route(
            "/allowed-subjects/:subject_id",
            put(add_preauthorize_subject),
        )
        .route("/generate-keys", get(register_keys))
        .route("/subjects", get(get_subjects))
        .route("/subjects/:subject_id", get(get_subject))
        .route(
            "/subjects/:subject_id/validation",
            get(get_validation_proof),
        )
        .route("/subjects/:subject_id/events", get(get_events_of_subject))
        .route(
            "/subjects/:subject_id/events/:sn",
            get(get_event_of_subject),
        )
        .layer(
            ServiceBuilder::new()
                .layer(Extension(ips_connects_state))
                .layer(axum::middleware::from_fn(limit_ip_request))
                .layer(Extension(kore_api)),
        );
    Router::new().merge(routes)
}
