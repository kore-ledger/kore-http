use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post, put},
    Extension, Json, Router,
};
use kore_bridge::{model::BridgeSignedEventRequest, Bridge};
use serde::Deserialize;
use serde_json::Value;
use tower::ServiceBuilder;
use utoipa::ToSchema;
use crate::{error::Error, wrappers::{GovsData, RegisterData, RequestDB, RequestData}};

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

#[cfg(feature = "doc")]
use crate ::doc::ApiDoc;
#[cfg(feature = "doc")]
use utoipa::OpenApi;
#[cfg(feature = "doc")]
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
#[ cfg_attr(feature = "doc", utoipa::path(
    post,
    path = "/event-request",
    operation_id = "Send Event Request",
    tag = "Requests",
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
))]
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
/// * `Result<Json<RequestDB>, Error>` - returns an Ok in a JSON or an error
#[cfg_attr(feature = "doc", utoipa::path(
    get,
    path = "/event-request/{request-id}",
    operation_id = "Get Request State",
    tag = "Requests",
    params(
        ("request-id" = String, Path, description = "Event Request's unique id"),
    ),
    responses(
        (status = 200, description = "Request Data successfully retrieved", body = RequestDB,
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
))]
async fn get_request_state(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(request_id): Path<String>,
) -> Result<Json<RequestDB>, Error> {
    match bridge.get_request_state(request_id).await {
        Ok(response) => Ok(Json(RequestDB::from(response))),
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
/// * `Result<Json<Value>, Error>` - returns an Ok in a JSON or an error
#[cfg_attr(feature = "doc", utoipa::path(
    get,
    path = "/approval-request/{subject_id}",
    operation_id = "Get one Approval Request Data",
    tag = "Approvals",
    params(
        ("subject_id" = String, Path, description = "subject unique id"),
    ),
    responses(
        (status = 200, description = "Approval Data successfully retrieved", body = Value),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
))]
async fn get_approval(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<Value>, Error> {
    match bridge.get_approval(subject_id).await {
        Ok(response) => Ok(Json(response)),
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
#[ cfg_attr(feature = "doc", utoipa::path(
    patch,
    path = "/approval-request/{subject_id}",
    operation_id = "Set your Aprroval for a request",
    tag = "Approvals",
    request_body(content = String, content_type = "application/json", description = "Vote of the user for an existing request"),
    params(
        ("subject_id" = String, Path, description = "Approval's unique id"),
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
))]
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

/// put authorization
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
#[ cfg_attr(feature = "doc",  utoipa::path(
    put,
    path = "/auth/{subject_id}",
    operation_id = "Put Authorization",
    tag = "Auth",
    request_body(content = String, content_type = "application/json", description = "witnesses"),
    params(
        ("subject_id" = String, Path, description = "Approval's unique id"),
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
))]
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
#[ cfg_attr(feature = "doc", utoipa::path(
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
))]
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
#[ cfg_attr(feature = "doc", utoipa::path(
    get,
    path = "/auth/{subject_id}",
    operation_id = "Get witnesses subject",
    tag = "Auth",
    params(
        ("subject_id" = String, Path, description = "Approval's unique id"),
    ),
    responses(
        (status = 200, description = "a list of witness nodes in Json", body = [String]),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
))]
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
#[ cfg_attr(feature = "doc", utoipa::path(
    delete,
    path = "/auth/{subject_id}",
    operation_id = "Delete authorized subjects",
    tag = "Auth",
    params(
        ("subject_id" = String, Path, description = "Approval's unique id"),
    ),
    responses(
        (status = 200, description = "Ok in JSON format", body = [String],
        example = json!(
            "Ok"
        )),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
))]
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
#[ cfg_attr(feature = "doc", utoipa::path(
    put,
    path = "/update/{subject_id}",
    operation_id = "Update Subject",
    tag = "Update",
    params(
        ("subject_id" = String, Path, description = "Approval's unique id"),
    ),
    responses(
        (status = 200, description = "Subject Data successfully retrieved", body = [String]),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
))]
async fn update_subject(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<String>, Error> {
    match bridge.update_subject(subject_id).await {
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
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/register-governances",
    operation_id = "Get All Governances",
    tag = "Governances",
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
))]
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
#[ cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/register-subjects/{governance_id}",
    operation_id = "Get All Subjects Data",
    tag = "Subjects",
    params(
        ("subject_id" = String, Path, description = "Approval's unique id"),
        ("parameters" = SubjectQuery, Query, description = "The query parameters for the request"),
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = [RegisterData]),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
))]
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
/// * `Result<Json<Value>, Error>` - A list of events in JSON format or an error if the request fails.
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/events/{subject_id}",
    operation_id = "Get Subject Events",
    tag = "Events",
    params(
        ("subject_id" = String, Path, description = "Approval's unique id"),
        ("parameters" = EventsQuery, Query, description = "The query parameters for the request"),
    ),
    responses(
        (status = 200, description = "Allows obtaining specific events of a subject by its identifier.", body = [Value],
        example = json!(
            {
                "events": [
                    {
                        "data": "[]",
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
))]
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
/// * `Result<Json<Value>, Error>` -the state of the subject in JSON format or an error if the request fails.
#[ cfg_attr(feature = "doc", utoipa::path(
    get,
    path = "/state/{subject_id}",
    operation_id = "Get Subject State",
    tag = "States",
    params(
        ("subject-id" = String, Path, description = "Subject's unique id"),
    ),
    responses(
        (status = 200, description = "Allows obtaining specific state of a subject by its identifier.", body = [Value],
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
))]
async fn get_state(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<Value>, Error> {
    match bridge.get_subject(subject_id).await {
        Ok(response) => Ok(Json(response)),
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
/// * `Result<Json<Value>, Error>` - the signature in JSON format or an error if the request fails.
#[ cfg_attr(feature = "doc", utoipa::path(
    get,
    path = "/signatures/{subject_id}",
    operation_id = "Get Subject Signatures",
    tag = "Signatures",
    params(
        ("subject-id" = String, Path, description = "Subject's unique id"),
    ),
    responses(
        (status = 200, description = "the signature in JSON format", body = [Value],
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
))]
async fn get_signatures(
    Extension(bridge): Extension<Arc<Bridge>>,
    Path(subject_id): Path<String>,
) -> Result<Json<Value>, Error> {
    match bridge.get_signatures(subject_id).await {
        Ok(response) => Ok(Json(response)),
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

#[ cfg_attr(feature = "doc", utoipa::path(
    get,
    path = "/controller-id",
    operation_id = "Get controller-id",
    tag = "Controller-id",
    responses(
        (status = 200, description = "Gets the controller id of the node",  body = String,
        example = json!(
            "E2ZY7GjU14U3m-iAqvhQM6kiG62uqLdBMBwv4J-4tzwI"
        )),
    )
))]
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

#[ cfg_attr(feature = "doc", utoipa::path(
    get,
    path = "/peer-id",
    operation_id = "Get peer-id",
    tag = "Peer-id",
    responses(
        (status = 200, description = "Gets the peer id of the node",  body = String,
        example = json!(
            "12D3KooWQTjWCGZa2f6ZVkwwcbEb4ghtS49AcssJSrATFBNxDpR7"
        )),
    )
))]
async fn get_peer_id(Extension(bridge): Extension<Arc<Bridge>>) -> Json<String> {
    Json(bridge.peer_id())
}

pub fn build_routes(bridge: Bridge) -> Router {
    let bridge = Arc::new(bridge);
    let routes=Router::new()
        .route("/signatures/{subject_id}", get(get_signatures))
        .route("/state/{subject_id}", get(get_state))
        .route("/events/{subject_id}", get(get_events))
        .route("/register-subjects/{governance_id}", get(get_all_subjects))
        .route("/register-governances", get(get_all_govs))
        .route("/update/{subject_id}", post(update_subject))
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

        #[cfg(feature = "doc")] {
            println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
            return Router::new().merge(routes).merge(
                RapiDoc::with_openapi("/doc/koreapi.json", ApiDoc::openapi()).path("/doc"),
        
            );
        }
        
        #[cfg(not(feature = "doc"))]
        Router::new().merge(routes)
    }

