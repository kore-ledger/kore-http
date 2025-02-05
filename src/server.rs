use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post, put},
    Extension, Json, Router,
};
use kore_bridge::{model::BridgeSignedEventRequest, Bridge};
use serde::Deserialize;
use tower::ServiceBuilder;
use utoipa::ToSchema;
use crate::{enviroment::build_doc, error::Error, wrappers::{ApproveInfo, EventInfo, GovsData, PaginatorEvents, RegisterData, RequestData, RequestInfo, SignaturesInfo, SubjectInfo}};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct SubjectQuery {
    active: Option<bool>,
    schema: Option<String>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct GovQuery {
    active: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct EventsQuery {
    quantity: Option<u64>,
    page: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct EventSnQuery {
    sn: u64,
}
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct EventFirstLastQuery {
    quantity: Option<u64>,
    reverse: Option<bool>,
    success: Option<bool>,
}

use crate ::doc::ApiDoc;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;


/// Send event request
///
/// Allows sending an event request for a subject to the Kore node.
/// These requests can be of any type of event (fact, creation, transfer, or end of life).
/// In case of external invocation, the requests can be signed.
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - The Bridge extension wrapped in an `Arc`.
/// * `Json(request): Json<BridgeSignedEventRequest>` - The signed event request in JSON format.
///
/// # Returns
///
/// * `Result<Json<RequestData>, Error>` - The response to the event request wrapped in a JSON object, or an error.
#[ utoipa::path(
    post,
    path = "/event-request",
    operation_id = "Send Event Request",
    tag = "Request",
    request_body(content = String, content_type = "application/json", description = "The signed event request"),
    responses(
        
        (status = 200, description = "Request Created Successfully", body = RequestData,
        example = json!(
            {
                "request_id":"JemKGBkBjpV5Q34zL-KItY9g-RuY4_QJIn0PpIjy0e_E",
                "subject_id":"Jd_vA5Dl1epomG7wyeHiqgKdOIBi28vNgHjRl6hy1N5w"
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 409, description = "Conflict"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn send_event_request(
    Extension(bridge): Extension<Arc<Bridge>>,
    Json(request): Json<BridgeSignedEventRequest>,
) -> Result<Json<RequestData>, Error> {
    match bridge.send_event_request(request).await {
        Ok(response) => Ok(Json(RequestData::from(response))),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

/// Get request state
///
/// Allows obtaining an event request by its identifier.
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<bridge>>` - The bridge extension wrapped in an `Arc`.
/// * `Path(request_id): Path<String>` - The identifier of the event request as a path parameter.
///
/// # Returns
///
/// * `Result<Json<RequestInfo>, Error>` - returns an Ok in a JSON or an error
#[utoipa::path(
    get,
    path = "/event-request/{request-id}",
    operation_id = "Get Request State",
    tag = "Request",
    params(
        ("request-id" = String, Path, description = "Event Request's unique id"),
    ),
    responses(
        (status = 200, description = "Request Data successfully retrieved", body = RequestInfo,
        example = json!(
            {
                "status": "Finish",
                "version": 0,
                "error": null
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn get_request_state(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(request_id): Path<String>,
) -> Result<Json<RequestInfo>, Error> {
    match bridge.get_request_state(request_id).await {
        Ok(response) => Ok(Json(RequestInfo::from(response))),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

/// Get approvals
///
/// Allows obtaining the list of requests for approvals received by the node.
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<bridge>>` - The bridge extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject as a path parameter.
///
/// # Returns
///
/// * `Result<Json<ApproveInfo>, Error>` - returns an Ok in a JSON or an error
#[utoipa::path(
    get,
    path = "/approval-request/{subject_id}",
    operation_id = "Get one Approval Request Data",
    tag = "Approval",
    params(
        ("subject_id" = String, Path, description = "Subjects unique id"),
    ),
    responses(
        (status = 200, description = "Approval Data successfully retrieved", body = ApproveInfo),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn get_approval(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<ApproveInfo>, Error> {
    match bridge.get_approval(subject_id).await {
        Ok(response) => Ok(Json(ApproveInfo::from(response))),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

/// patch approval
///
/// Allows issuing an affirmative or negative approval for a previously received request.
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<bridge>>` - The bridge extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` -The identifier of the subject as a path parameter.
/// * `Json(response): Json<String>` - The response (approval or rejection) in JSON format
/// 
/// # Returns
///
/// * `Result<Json<String>, Error>` - The approval request in JSON format or an error if the request fails.
#[ utoipa::path(
    patch,
    path = "/approval-request/{subject_id}",
    operation_id = "Set your Aprroval for a request",
    tag = "Approval",
    request_body(content = String, content_type = "application/json", description = "Vote of the user for an existing request"),
    params(
        ("subject_id" = String, Path, description = "Subjects unique id"),
    ),
    responses(
        (status = 200, description = "Request successfully voted", body = String,
        example = json!(
            "The approval request for subject Jd_vA5Dl1epomG7wyeHiqgKdOIBi28vNgHjRl6hy1N5w has changed to RespondedAccepted"
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 409, description = "Conflict"),
        (status = 500, description = "Internal Server Error"),
    )
)]
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

/// Put authorization
///
/// Given a subject identifier and one or more witnesses, the witnesses authorize the subject to send them copy of the logs
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<bridge>>` - The bridge extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject to be authorized as a path parameter.
/// * `Json(witnesses): Json<Vec<String>>` - The witnesses who will receive the copy of the logs in JSON format
///
/// # Returns
///
/// * `Result<Json<String>, Error>` - The result of the approval as a JSON object or an error if the request fails.
#[  utoipa::path(
    put,
    path = "/auth/{subject_id}",
    operation_id = "Put Authorization",
    tag = "Auth",
    request_body(content = String, content_type = "application/json", description = "witnesses"),
    params(
        ("subject_id" = String, Path, description = "Subjects unique id"),
    ),
    responses(
        (status = 200, description = "The result of the approval as a JSON object", body = String,
        example = json!(
            "Ok"
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 409, description = "Conflict"),
        (status = 500, description = "Internal Server Error"),
    )
)]
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

/// Get authorized subjects
///
/// Allows obtaining the list of subjects that have been authorized by the node
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - The bridge extension wrapped in an `Arc`.
///
/// # Returns
///
/// * `Result<Json<Vec<String>>, Error>` - A list of authorized subjects in JSON format or an error if the request fails.
#[ utoipa::path(
    get,
    path = "/auth",
    operation_id = "Get authorized subjects",
    tag = "Auth",
    responses(
        (status = 200, description = "A list of authorized subjects in JSON ", body = [String],
        example = json!(
            [
                "J6blziscpjD0pJXsRh6_ooPtBsvwEZhx-xO4hT7WoKg0"
            ]
        )),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn get_all_auth_subjects(
    Extension(bridge): Extension<Arc<Bridge>>,
) -> Result<Json<Vec<String>>, Error> {
    match bridge.get_all_auth_subjects().await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

/// Get witnesses subject
///
/// Obtains a subject's witnesses
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - The bridge extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject as a path parameter.
///
/// # Returns
///
/// * `Result<Json<Vec<String>>, Error>` - a list of witness nodes in Json format or an error
#[ utoipa::path(
    get,
    path = "/auth/{subject_id}",
    operation_id = "Get witnesses subject",
    tag = "Auth",
    params(
        ("subject_id" = String, Path, description = "Subjects unique id"),
    ),
    responses(
        (status = 200, description = "a list of witness nodes in Json", body = [String]),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn get_witnesses_subject(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<Vec<String>>, Error> {
    match bridge.get_witnesses_subject(subject_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

/// Delete authorized subjects
///
/// Deletes an authorized subject given its identifier
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - bridge extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject as a path parameter.
///
/// # Returns
///
/// * `Result<Json<String>, Error>` - Ok in JSON format or an error if the request fails.
#[ utoipa::path(
    delete,
    path = "/auth/{subject_id}",
    operation_id = "Delete authorized subjects",
    tag = "Auth",
    params(
        ("subject_id" = String, Path, description = "Subjects unique id"),
    ),
    responses(
        (status = 200, description = "Ok in JSON format", body = String,
        example = json!(
            "Ok"
        )),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn delete_auth_subject(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<String>, Error> {
    match bridge.delete_auth_subject(subject_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

/// Update Subject
///
/// Updates an authorized subject given its identifier
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - bridge extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject as a path parameter.
///
/// # Returns
///
/// * `Result<Json<String>, Error>` - A message in JSON format or an error if the request fails.
#[ utoipa::path(
    post,
    path = "/update/{subject_id}",
    operation_id = "Update Subject",
    tag = "Update",
    params(
        ("subject_id" = String, Path, description = "Subjects unique id"),
    ),
    responses(
        (status = 200, description = "Subject Data successfully retrieved", body = String),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn update_subject(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<String>, Error> {
    match bridge.update_subject(subject_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

/// Check Transfer
///
/// Check transfer event for subject given its identifier
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - bridge extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject as a path parameter.
///
/// # Returns
///
/// * `Result<Json<String>, Error>` - A message in JSON format or an error if the request fails.
#[ utoipa::path(
    post,
    path = "/check_transfer/{subject_id}",
    operation_id = "Check transfer",
    tag = "Transfer",
    params(
        ("subject_id" = String, Path, description = "Subjects unique id"),
    ),
    responses(
        (status = 200, description = "Subject Data successfully retrieved", body = String),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn check_transfer(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<String>, Error> {
    match bridge.check_transfer(subject_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

/// Manual Update
///
/// Throw to witnesses the last distribution of a subject
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - bridge extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject as a path parameter.
///
/// # Returns
///
/// * `Result<Json<String>, Error>` - A message in JSON format or an error if the request fails.
#[ utoipa::path(
    post,
    path = "/manual-distribution/{subject_id}",
    operation_id = "Manual Update Subject",
    tag = "Update",
    params(
        ("subject_id" = String, Path, description = "Subjects unique id"),
    ),
    responses(
        (status = 200, description = "Subject Data successfully retrieved", body = String),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn manual_distribution(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<String>, Error> {
    match bridge.manual_distribution(subject_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}
/// Get all gov
///
/// Gets all the governorships to which the node belongs
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - bridge extension wrapped in an `Arc`.
/// * `Query(parameters): Query<GovQuery>` - The query parameters for the request.
///
/// # Returns
///
/// * `Result<Json<Vec<GovsData>>, Error>` - A JSON with governance information or an error if the request fails.
#[ utoipa::path(
    get,
    path = "/register-governances",
    operation_id = "Get All Governances",
    tag = "Governance",
    params(
        ("parameters" = GovQuery, Query, description = "The query parameters for the request"),
    ),
    responses(
        (status = 200, description = "Gets all the governorships to which the node belongs", body = [GovsData],
        example = json!(
            [
                {
                    "governance_id": "JUH9HGYpqMgN3D3Wb43BCPKdb38K1ocDauupuvCN0plM",
                    "active": true
                },
                {
                    "governance_id": "Jl9LVUi8uVBmV9gitxEiiVeSWxEceZoOYT-Kx-t9DTVE",
                    "active": true
                }
            ]
        )),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn get_all_govs(
    Extension(bridge): Extension<Arc<Bridge>>,
    Query(parameters): Query<GovQuery>,
) -> Result<Json<Vec<GovsData>>, Error> {
    match bridge.get_all_govs(parameters.active).await {
        Ok(response) => Ok(Json(response.iter().map(|x| GovsData::from(x.clone())).collect())),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

/// Get all subjects
///
/// Allows obtaining the list of subjects known by the node with pagination.
/// It can also be used to obtain only the governances and all subjects belonging to a specific governance.
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - The bridge extension wrapped in an `Arc`.
/// * `Path(governance_id): Path<String>` - The identifier of the governance as a path parameter.
/// * `Query(parameters): Query<SubjectQuery>` - The query parameters for the request.
/// 
/// # Returns
///
/// * `Result<Json<Vec<RegisterData>>, Error>` - A list of subjects in JSON format or an error if the request fails.
#[  utoipa::path(
    get,
    path = "/register-subjects/{governance_id}",
    operation_id = "Get All Subjects Data",
    tag = "Subject",
    params(
        ("subject_id" = String, Path, description = "Subjects unique id"),
        ("parameters" = SubjectQuery, Query, description = "The query parameters for the request"),
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = [RegisterData]),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn get_all_subjects(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(governance_id): Path<String>,
    Query(parameters): Query<SubjectQuery>,
) -> Result<Json<Vec<RegisterData>>, Error> {
    match bridge
        .get_all_subjs(governance_id, parameters.active, parameters.schema)
        .await
    {
        Ok(response) => Ok(Json(response.iter().map(|x| RegisterData::from(x.clone())).collect())),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

/// Get events of a subject
///
/// Allows obtaining specific events of a subject by its identifier.
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - The bridge extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject as a path parameter.
/// * `Query(parameters): Query<EventsQuery>` - The pagination parameters for the request.
///
/// # Returns
///
/// * `Result<Json<PaginatorEvents>, Error>` - A list of events in JSON format or an error if the request fails.
#[ utoipa::path(
    get,
    path = "/events/{subject_id}",
    operation_id = "Get Subject Events",
    tag = "Event",
    params(
        ("subject_id" = String, Path, description = "Subjects unique id"),
        ("parameters" = EventsQuery, Query, description = "The query parameters for the request"),
    ),
    responses(
        (status = 200, description = "Allows obtaining specific events of a subject by its identifier.", body = [PaginatorEvents],
        example = json!(
            {
                "events": [
                    {
                        "patch": "[]",
                        "error": null,
                        "event_req": {
                            "Create": {
                                "governance_id": "",
                                "namespace": [],
                                "schema_id": "governance"
                            }
                        },
                        "sn": 0,
                        "subject_id": "Jd_vA5Dl1epomG7wyeHiqgKdOIBi28vNgHjRl6hy1N5w",
                        "succes": true
                    }
                ],
                "paginator": {
                    "next": null,
                    "pages": 1,
                    "prev": null
                }
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn get_events(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
    Query(parameters): Query<EventsQuery>,
) -> Result<Json<PaginatorEvents>, Error> {
    match bridge
        .get_events(subject_id, parameters.quantity, parameters.page)
        .await
    {
        Ok(response) => Ok(Json(PaginatorEvents::from(response))),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

/// Get state of a subject
///
/// Allows obtaining specific state of a subject by its identifier.
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - bridge extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject as a path parameter.
///
/// # Returns
///
/// * `Result<Json<SubjectInfo>, Error>` -the state of the subject in JSON format or an error if the request fails.
#[utoipa::path(
    get,
    path = "/state/{subject_id}",
    operation_id = "Get Subject State",
    tag = "State",
    params(
        ("subject-id" = String, Path, description = "Subject's unique id"),
    ),
    responses(
        (status = 200, description = "Allows obtaining specific state of a subject by its identifier.", body = [SubjectInfo],
        example = json!(
            {
                "active": true,
                "creator": "E2ZY7GjU14U3m-iAqvhQM6kiG62uqLdBMBwv4J-4tzwI",
                "genesis_gov_version": 0,
                "governance_id": "",
                "namespace": "",
                "owner": "E2ZY7GjU14U3m-iAqvhQM6kiG62uqLdBMBwv4J-4tzwI",
                "properties": {
                    "members": [
                        {
                            "id": "E2ZY7GjU14U3m-iAqvhQM6kiG62uqLdBMBwv4J-4tzwI",
                            "name": "Owner"
                        }
                    ],
                    "policies": [
                        {
                            "approve": {
                                "quorum": "MAJORITY"
                            },
                            "evaluate": {
                                "quorum": "MAJORITY"
                            },
                            "id": "governance",
                            "validate": {
                                "quorum": "MAJORITY"
                            }
                        }
                    ],
                    "roles": [
                        {
                            "namespace": "",
                            "role": "WITNESS",
                            "schema": {
                                "ID": "governance"
                            },
                            "who": "MEMBERS"
                        },
                        {
                            "namespace": "",
                            "role": "EVALUATOR",
                            "schema": "ALL",
                            "who": {
                                "NAME": "Owner"
                            }
                        },
                        {
                            "namespace": "",
                            "role": "ISSUER",
                            "schema": {
                                "ID": "governance"
                            },
                            "who": {
                                "NAME": "Owner"
                            }
                        },
                        {
                            "namespace": "",
                            "role": "APPROVER",
                            "schema": {
                                "ID": "governance"
                            },
                            "who": {
                                "NAME": "Owner"
                            }
                        },
                        {
                            "namespace": "",
                            "role": "VALIDATOR",
                            "schema": "ALL",
                            "who": {
                                "NAME": "Owner"
                            }
                        },
                        {
                            "namespace": "",
                            "role": "WITNESS",
                            "schema": "ALL",
                            "who": {
                                "NAME": "Owner"
                            }
                        }
                    ],
                    "schemas": [],
                    "version": 0
                },
                "schema_id": "governance",
                "sn": 0,
                "subject_id": "Jd_vA5Dl1epomG7wyeHiqgKdOIBi28vNgHjRl6hy1N5w"
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn get_state(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<SubjectInfo>, Error> {
    match bridge.get_subject(subject_id).await {
        Ok(response) => Ok(Json(SubjectInfo::from(response))),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

/// Get signatures of a subject
///
/// Allows obtaining signatures of the last event of subject.
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - The bridge extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject as a path parameter.
///
/// # Returns
///
/// * `Result<Json<SignaturesInfo>, Error>` - the signature in JSON format or an error if the request fails.
#[ utoipa::path(
    get,
    path = "/signatures/{subject_id}",
    operation_id = "Get Subject Signatures",
    tag = "Signature",
    params(
        ("subject-id" = String, Path, description = "Subject's unique id"),
    ),
    responses(
        (status = 200, description = "the signature in JSON format", body = [SignaturesInfo],
        example = json!(
            {
                "signatures_appr": null,
                "signatures_eval": null,
                "signatures_vali": [
                    {
                        "Signature": {
                            "content_hash": "JLZZ0vv3xwydlcUSIyS2r1J3f8Gz9R03i6ofLTwltheE",
                            "signer": "E2ZY7GjU14U3m-iAqvhQM6kiG62uqLdBMBwv4J-4tzwI",
                            "timestamp": 17346911,
                            "value": "SEySTR3fRiBzlps2Zc3r-Yb8HMiCV5kZJtAu7DYt4xczN8ogW5AZhVjhn6EOj3DmsNyBeFaGIHQrnVnPxA8vkBDA"
                        }
                    }
                ],
                "sn": 0,
                "subject_id": "Jd_vA5Dl1epomG7wyeHiqgKdOIBi28vNgHjRl6hy1N5w"
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn get_signatures(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<SignaturesInfo>, Error> {
    match bridge.get_signatures(subject_id).await {
        Ok(response) => Ok(Json(SignaturesInfo::from(response))),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

/// Get controller-id
///
/// Gets the controller id of the node
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - The bridge extension wrapped in an `Arc`.
///
/// # Returns
///
/// * `Json<String>` - Returns the controller-id of the node in a Json

#[ utoipa::path(
    get,
    path = "/controller-id",
    operation_id = "Get controller-id",
    tag = "Other",
    responses(
        (status = 200, description = "Gets the controller id of the node",  body = String,
        example = json!(
            "E2ZY7GjU14U3m-iAqvhQM6kiG62uqLdBMBwv4J-4tzwI"
        )),
    )
)]
async fn get_controller_id(Extension(bridge): Extension<Arc<Bridge>>) -> Json<String> {
    Json(bridge.controller_id())
}

/// Get peer-id
///
/// Gets the peer id of the node
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - The bridge extension wrapped in an `Arc`.
///
/// # Returns
///
/// * `Json<String>` - Returns the peer id of the node in a Json
#[ utoipa::path(
    get,
    path = "/peer-id",
    operation_id = "Get peer-id",
    tag = "Other",
    responses(
        (status = 200, description = "Gets the peer id of the node",  body = String,
        example = json!(
            "12D3KooWQTjWCGZa2f6ZVkwwcbEb4ghtS49AcssJSrATFBNxDpR7"
        )),
    )
)]
async fn get_peer_id(Extension(bridge): Extension<Arc<Bridge>>) -> Json<String> {
    Json(bridge.peer_id())
}

/// Get events of a sn
///
/// Allows obtaining specific events of a subject by its identifier and sn.
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - The bridge extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject as a path parameter.
/// * `Query(parameters): Query<EventSnQuery>` - The query parameters for the request.
///
/// # Returns
///
/// * `Result<Json<EventInfo>, Error>` - A list of events in JSON format or an error if the request fails.
#[utoipa::path(
    get,
    path = "/event/{subject_id}",
    operation_id = "Get Subject Events with sn",
    tag = "Event",
    params(
        ("subject_id" = String, Path, description = "Subjects unique id"),
        ("parameters" = EventSnQuery, Query, description = "The query parameters for the request"),
    ),
    responses(
        (status = 200, description = "Allows obtaining specific events of a subject by its identifier and sn", body = [EventInfo],
        example = json!(
            {
                "events": [
                    {
                        "patch": "[]",
                        "error": null,
                        "event_req": {
                            "Create": {
                                "governance_id": "",
                                "namespace": [],
                                "schema_id": "governance"
                            }
                        },
                        "sn": 0,
                        "subject_id": "Jd_vA5Dl1epomG7wyeHiqgKdOIBi28vNgHjRl6hy1N5w",
                        "succes": true
                    }
                ],
                "paginator": {
                    "next": null,
                    "pages": 1,
                    "prev": null
                }
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn get_event_sn(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
    Query(parameters): Query<EventSnQuery>,
) -> Result<Json<EventInfo>, Error> {
    match bridge
        .get_event_sn(subject_id, parameters.sn)
        .await
    {
        Ok(response) => Ok(Json(EventInfo::from(response))),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

/// Get first or last events of a subject
///
/// Given a subject id a specific number of events can be obtained, depending on the quantity, reverse and success parameters.
///
/// # Parameters
///
/// * `Extension(bridge): Extension<Arc<Bridge>>` - The bridge extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject as a path parameter.
/// * `Query(parameters): Query<EventFirstLastQuery>` - The query parameters for the request.
///
/// # Returns
///
/// * `Result<Json<EventInfo>, Error>` - A list of events in JSON format or an error if the request fails.
#[utoipa::path(
    get,
    path = "/events-first-last/{subject_id}",
    operation_id = "Get specifics Subject Events",
    tag = "Event",
    params(
        ("subject_id" = String, Path, description =  "Subjects unique id"),
        ("parameters" = EventFirstLastQuery, Query, description = "The query parameters for the request"),
    ),
    responses(
        (status = 200, description = "Allows obtaining specific events of a subject by its identifier and sn", body = [EventInfo],
        example = json!(
            {
                "events": [
                    {
                        "patch": "[]",
                        "error": null,
                        "event_req": {
                            "Create": {
                                "governance_id": "",
                                "namespace": [],
                                "schema_id": "governance"
                            }
                        },
                        "sn": 0,
                        "subject_id": "Jd_vA5Dl1epomG7wyeHiqgKdOIBi28vNgHjRl6hy1N5w",
                        "succes": true
                    }
                ],
                "paginator": {
                    "next": null,
                    "pages": 1,
                    "prev": null
                }
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
)]
async fn get_first_or_end_events(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
    Query(parameters): Query<EventFirstLastQuery>,
) -> Result<Json<Vec<EventInfo>>, Error> {
    match bridge
        .get_first_or_end_events(subject_id, parameters.quantity, parameters.reverse, parameters.success)
        .await
    {
        Ok(response) => Ok(Json(response.iter().map(|x| EventInfo::from(x.clone())).collect())),
        Err(e) => Err(Error::Kore(e.to_string())),
    }
}

pub fn build_routes(bridge: Bridge) -> Router {
    let bridge = Arc::new(bridge);
    let routes=Router::new()
        .route("/signatures/{subject_id}", get(get_signatures))
        .route("/state/{subject_id}", get(get_state))
        .route("/events/{subject_id}", get(get_events))
        .route("/event/{subject_id}", get(get_event_sn))
        .route("/events-first-last/{subject_id}", get(get_first_or_end_events))
        .route("/register-subjects/{governance_id}", get(get_all_subjects))
        .route("/register-governances", get(get_all_govs))
        .route("/update/{subject_id}", post(update_subject))
        .route("/check-transfer/{subject_id}", post(check_transfer))
        .route("/manual-distribution/{subject_id}", post(manual_distribution))
        .route("/auth/{subject_id}", delete(delete_auth_subject))
        .route("/auth/{subject_id}", get(get_witnesses_subject))
        .route("/auth", get(get_all_auth_subjects))
        .route("/auth/{subject_id}", put(put_auth))
        .route("/approval-request/{subject_id}", patch(patch_approval))
        .route("/approval-request/{subject_id}", get(get_approval))
        .route("/event-request/{request_id}", get(get_request_state))
        .route("/event-request", post(send_event_request))
        .route("/controller-id", get(get_controller_id))
        .route("/peer-id", get(get_peer_id))
        .layer(ServiceBuilder::new().layer(Extension(bridge)));

        if build_doc() {
            Router::new().merge(routes).merge(
                RapiDoc::with_openapi("/doc/koreapi.json", ApiDoc::openapi()).path("/doc"),
            )
        } else {
            Router::new().merge(routes)
        }
    }
