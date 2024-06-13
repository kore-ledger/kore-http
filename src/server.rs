use axum::{
    extract::{Path, Query},
    routing::{get, patch, post, put},
    Extension, Json, Router,
};
use kore_node::{model::*, KoreApi};
use log::debug;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tower::ServiceBuilder;

#[cfg(feature = "doc")]
use crate::doc::utoipa::ApiDoc;
use crate::{common::IPSMaxConnectState, error::Errors, middleware::middlewares::limit_ip_request};
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
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
/// * `Json(request): Json<NodeSignedEventRequest>` - The signed event request in JSON format.
///
/// # Returns
///
/// * `Result<Json<EventRequestResponse>, Errors>` - The response to the event request wrapped in a JSON object, or an error.
#[cfg_attr(feature = "doc", utoipa::path(
    post,
    path = "/event-requests",
    tag = "Requests",
    operation_id = "Create Event Request",
    request_body = NodeSignedEventRequest,
    responses(
        (status = 200, description = "Request Created Successfully", body = EventRequestResponse,
        example = json!(
            {
                "request_id": "J8618wGO7hH4wRuEeL0Ob5XNI9Q73BlCNlV8cWBORq78"
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 409, description = "Conflict"),
        (status = 500, description = "Internal Server Error"),
    )
))]
async fn send_event_request(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Json(request): Json<NodeSignedEventRequest>,
) -> Result<Json<EventRequestResponse>, Errors> {
    match kore_api.send_event_request(request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

/// Get event request
///
/// Allows obtaining an event request by its identifier.
///
/// # Parameters
///
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
/// * `Path(request_id): Path<String>` - The identifier of the event request as a path parameter.
///
/// # Returns
///
/// * `Result<Json<NodeSignedEventRequest>, Errors>` - The requested event in JSON format or an error if the request fails.
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/event-requests/{request-id}",
    operation_id = "Get Event Request Data",
    tag = "Requests",
    params(
        ("request-id" = String, Path, description = "Event Request's unique id"),
    ),
    responses(
        (status = 200, description = "Request Data successfully retrieved", body = NodeSignedEventRequest,
        example = json!(
            {
                "request": {
                  "Fact": {
                    "subject_id": "Jz_XWeQtVjhoKxoeQCBHSnLlK-WGutaddyT5zpwaNAsI",
                    "payload": {
                      "Patch": {
                        "data": [
                          {
                            "op": "add",
                            "path": "/members/1",
                            "value": {
                              "id": "EyyisBz0lX9sRvvV0H-BXTrVtARjUa0YDHzaxFHWH-N4",
                              "name": "Tutorial2"
                            }
                          }
                        ]
                      }
                    }
                  }
                },
                "signature": {
                  "signer": "EwkWURnRVk-lUEjF0cowczxYkz8DpbhLfo3UMSZE00LE",
                  "timestamp": 171827,
                  "value": "SEBSFU_WMTWlyOU6hzzZwlNUMJ8cJHD_GDXBU6NPUJfpikjlocv-sGra2aogrufjQdI1IfxAl0uN4jpTKGlRxeBA",
                  "content_hash": "JbALAEwzWEud0462GY-WB6JPHH1Ow1pCUGtPAjjB5uq8"
                }
              }
        )
    ),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
))]
async fn get_event_request(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Path(request_id): Path<String>,
) -> Result<Json<NodeSignedEventRequest>, Errors> {
    match kore_api.get_event_request(&request_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

/// Get event request state
///
/// Allows obtaining the status of an event request by its identifier.
///
/// # Parameters
///
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
/// * `Path(request_id): Path<String>` - The identifier of the event request.
///
/// # Returns
///
/// * `Result<Json<NodeKoreRequestState>, Errors>` - The status of the event request as a JSON object or an error if the request fails.
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/event-requests/{request-id}/state",
    operation_id = "Get Event Request State Data",
    tag = "Requests",
    params(
        ("request-id" = String, Path, description = "Event Request's unique id"),
    ),
    responses(
        (status = 200, description = "Request Data successfully retrieved", body = NodeKoreRequestState,
        example = json!(
            {
                "id": "JyyWIjUa3Ui04oTSN4pJFT8FhmgPRPXzsG4_tIX8IBFg",
                "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                "sn": 1,
                "state": "finished",
                "success": true
            }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
))]
async fn get_event_request_state(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Path(request_id): Path<String>,
) -> Result<Json<NodeKoreRequestState>, Errors> {
    match kore_api.get_event_request_state(&request_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

/// Get approvals
///
/// Allows obtaining the list of requests for approvals received by the node.
/// It can also be used, by means of the "status" parameter, to list the requests pending approval.
///
/// # Parameters
///
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
/// * `Query(parameters): Query<NodeGetApprovals>` - The query parameters including the status for filtering.
///
/// # Returns
///
/// * `Result<Json<Vec<NodeApprovalEntity>>, Errors>` - A list of approval requests in JSON format or an error if the request fails.
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/approval-requests",
    operation_id = "Get all Approvals Request Data",
    tag = "Approvals",
    params(
        ("status" = Option<String>, Query, description = "Approval's status (possibilities: pending, obsolete, responded_accepted,responded_rejected)"),
        ("from" = Option<String>, Query, description = "Id of initial approval"),
        ("quantity" = Option<i64>, Query, description = "Quantity of approvals requested"
    )),
    responses(
        (status = 200, description = "Approvals Data successfully retrieved", body = [NodeApprovalEntity],
        example = json!(
            [
                {
                    "id": "JOuIZRXB983t9w9lAEdjXRGAf9r9WX14TajGnni_5q5Y",
                    "request": {
                    "event_request": {
                        "request": {
                        "Fact": {
                            "subject_id": "Jz_XWeQtVjhoKxoeQCBHSnLlK-WGutaddyT5zpwaNAsI",
                            "payload": {
                            "Patch": {
                                "data": [
                                {
                                    "op": "add",
                                    "path": "/members/1",
                                    "value": {
                                    "id": "EyyisBz0lX9sRvvV0H-BXTrVtARjUa0YDHzaxFHWH-N4",
                                    "name": "Tutorial2"
                                    }
                                }
                                ]
                            }
                            }
                        }
                        },
                        "signature": {
                        "signer": "EwkWURnRVk-lUEjF0cowczxYkz8DpbhLfo3UMSZE00LE",
                        "timestamp": 1718270,
                        "value": "SEBSFU_WMTWlyOU6hzzZwlNUMJ8cJHD_GDXBU6NPUJfpikjlocv-sGra2aogrufjQdI1IfxAl0uN4jpTKGlRxeBA",
                        "content_hash": "JbALAEwzWEud0462GY-WB6JPHH1Ow1pCUGtPAjjB5uq8"
                        }
                    },
                    "sn": 2,
                    "gov_version": 1,
                    "patch": [
                        {
                        "op": "add",
                        "path": "/members/1",
                        "value": {
                            "id": "EyyisBz0lX9sRvvV0H-BXTrVtARjUa0YDHzaxFHWH-N4",
                            "name": "Tutorial2"
                        }
                        }
                    ],
                    "state_hash": "JjeSDJcWmnedQnledSlGv46ZVa3GnhLz7jE_aVFXz_hQ",
                    "hash_prev_event": "JRO73PuZwEzXPne5mv4Oe4qDj4elhsU0b6AlHKs7-cTs",
                    "signature": {
                        "signer": "E2MJmrdcSC827EPFFHf_J4lGgcLcrwyEmPCyAbqTlc-w",
                        "timestamp": 1718270,
                        "value": "SE9X0_Ytp2QQwQxn66JqYoyrrNmA7-U2gl5-mhWnc1XDDAcT4W_iyx7y0CSyQ3nCRfgn1pf7rFF6a1yA77Sf6aBA",
                        "content_hash": "JcMfxnVhkr86gIh2_ZQwIEQvcavw9wcVfhiOVY1CYzMY"
                    }
                    },
                    "reponse": null,
                    "state": "Pending"
                },
            ]
        )),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
))]
async fn get_approvals(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Query(parameters): Query<NodeGetApprovals>,
) -> Result<Json<Vec<NodeApprovalEntity>>, Errors> {
    match kore_api.get_approvals(parameters).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

/// Get approval by ID
///
/// Allows obtaining a request for approval by its identifier.
///
/// # Parameters
///
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
/// * `Path(id): Path<String>` - The identifier of the approval request.
///
/// # Returns
///
/// * `Result<Json<NodeApprovalEntity>, Errors>` - The approval request in JSON format or an error if the request fails.
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/approval-requests/{id}",
    operation_id = "Get one Approval Request Data",
    tag = "Approvals",
    params(
        ("id" = String, Path, description = "Approval's unique id")
    ),
    responses(
        (status = 200, description = "Approval Data successfully retrieved", body = NodeApprovalEntity,
        example = json!(
            {
                "id": "JOuIZRXB983t9w9lAEdjXRGAf9r9WX14TajGnni_5q5Y",
                "request": {
                  "event_request": {
                    "request": {
                      "Fact": {
                        "subject_id": "Jz_XWeQtVjhoKxoeQCBHSnLlK-WGutaddyT5zpwaNAsI",
                        "payload": {
                          "Patch": {
                            "data": [
                              {
                                "op": "add",
                                "path": "/members/1",
                                "value": {
                                  "id": "EyyisBz0lX9sRvvV0H-BXTrVtARjUa0YDHzaxFHWH-N4",
                                  "name": "Tutorial2"
                                }
                              }
                            ]
                          }
                        }
                      }
                    },
                    "signature": {
                      "signer": "EwkWURnRVk-lUEjF0cowczxYkz8DpbhLfo3UMSZE00LE",
                      "timestamp": 1718270,
                      "value": "SEBSFU_WMTWlyOU6hzzZwlNUMJ8cJHD_GDXBU6NPUJfpikjlocv-sGra2aogrufjQdI1IfxAl0uN4jpTKGlRxeBA",
                      "content_hash": "JbALAEwzWEud0462GY-WB6JPHH1Ow1pCUGtPAjjB5uq8"
                    }
                  },
                  "sn": 2,
                  "gov_version": 1,
                  "patch": [
                    {
                      "op": "add",
                      "path": "/members/1",
                      "value": {
                        "id": "EyyisBz0lX9sRvvV0H-BXTrVtARjUa0YDHzaxFHWH-N4",
                        "name": "Tutorial2"
                      }
                    }
                  ],
                  "state_hash": "JjeSDJcWmnedQnledSlGv46ZVa3GnhLz7jE_aVFXz_hQ",
                  "hash_prev_event": "JRO73PuZwEzXPne5mv4Oe4qDj4elhsU0b6AlHKs7-cTs",
                  "signature": {
                    "signer": "E2MJmrdcSC827EPFFHf_J4lGgcLcrwyEmPCyAbqTlc-w",
                    "timestamp": 1718270,
                    "value": "SE9X0_Ytp2QQwQxn66JqYoyrrNmA7-U2gl5-mhWnc1XDDAcT4W_iyx7y0CSyQ3nCRfgn1pf7rFF6a1yA77Sf6aBA",
                    "content_hash": "JcMfxnVhkr86gIh2_ZQwIEQvcavw9wcVfhiOVY1CYzMY"
                  }
                },
                "reponse": null,
                "state": "Pending"
              }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
))]
async fn get_approval_id(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Path(id): Path<String>,
) -> Result<Json<NodeApprovalEntity>, Errors> {
    match kore_api.get_approval_id(&id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

/// Emit approval for request
///
/// Allows issuing an affirmative or negative approval for a previously received request.
///
/// # Parameters
///
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
/// * `Path(id): Path<String>` - The identifier of the approval request.
/// * `Json(vote): Json<PatchVote>` - The vote (approval or rejection) in JSON format.
///
/// # Returns
///
/// * `Result<Json<NodeApprovalEntity>, Errors>` - The result of the approval as a JSON object or an error if the request fails.
#[cfg_attr(feature = "doc",  utoipa::path(
    patch,
    path = "/approval-requests/{id}",
    operation_id = "Set your Aprroval for a request",
    tag = "Approvals",
    request_body(content = PatchVote, content_type = "application/json", description = "Vote of the user for an existing request",
    example = json!(
        {"state": "RespondedAccepted"}
    )),
    params(
        ("id" = String, Path, description = "Approval's unique id"),
    ),
    responses(
        (status = 200, description = "Request successfully voted", body = NodeApprovalEntity,
        example = json!(
            {
                "id": "JSdXPN_MY0Oxue-J3pB4j47-bb296a-KRpZL9o5u4dNo",
                "request": {
                  "event_request": {
                    "request": {
                      "Fact": {
                        "subject_id": "Jz_XWeQtVjhoKxoeQCBHSnLlK-WGutaddyT5zpwaNAsI",
                        "payload": {
                          "Patch": {
                            "data": [
                              {
                                "op": "add",
                                "path": "/members/0",
                                "value": {
                                  "id": "EnyisBz0lX9sRvvV0H-BXTrVtARjUa0YDHzaxFHWH-N4",
                                  "name": "Tutorial1"
                                }
                              }
                            ]
                          }
                        }
                      }
                    },
                    "signature": {
                      "signer": "EwkWURnRVk-lUEjF0cowczxYkz8DpbhLfo3UMSZE00LE",
                      "timestamp": 17182700,
                      "value": "SEXiXQa7fwShQO7nN3s6o0jH-R5JZ_UGeLU1mhbYzUuF4ujByWVTxorbkonEkAEa3nf3ay-vsRJzoHlbrcxnwCDA",
                      "content_hash": "JNKGHoEQ8MV9nqmp_xRDfIMCH2-2B2V-dbNObTmdvJjw"
                    }
                  },
                  "sn": 1,
                  "gov_version": 0,
                  "patch": [
                    {
                      "op": "add",
                      "path": "/members/0",
                      "value": {
                        "id": "EnyisBz0lX9sRvvV0H-BXTrVtARjUa0YDHzaxFHWH-N4",
                        "name": "Tutorial1"
                      }
                    }
                  ],
                  "state_hash": "J9ZorCKUeboco5eBZeW_NYssO3ZYLu2Ano_tThl8_Fss",
                  "hash_prev_event": "JItOA_80oGJGbKuxd37Rhiv4GtojfK67v-a39RNlQoIg",
                  "signature": {
                    "signer": "E2MJmrdcSC827EPFFHf_J4lGgcLcrwyEmPCyAbqTlc-w",
                    "timestamp": 17182700,
                    "value": "SExa-v-XyA6skhRMH4dxy7a0Sraiw04aMOAvuo5TpMf8YGs-6j6bEy_KPV5Auc4LF35q5nqmy3FVTYKmiHSx4hCw",
                    "content_hash": "JRQTYPHiKfHsL-IVYu1I0PstSdt86C6V-_3MnSmfDQGk"
                  }
                },
                "reponse": {
                  "appr_req_hash": "JSdXPN_MY0Oxue-J3pB4j47-bb296a-KRpZL9o5u4dNo",
                  "approved": true,
                  "signature": {
                    "signer": "EwkWURnRVk-lUEjF0cowczxYkz8DpbhLfo3UMSZE00LE",
                    "timestamp": 17182700,
                    "value": "SEj2jBPWPvKWuT3fYAA9VdMObJnODo-lmL5t3y0Kh6jepdrB9BA4D_G5E54GOxcGdERGVAdYa-olieFd96HWEiCA",
                    "content_hash": "JOt8sRYMnz6vhDA_dDs8gvv8J0mqQt8MlQO-5Ktt-ENE"
                  }
                },
                "state": "RespondedAccepted"
              }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 409, description = "Conflict"),
        (status = 500, description = "Internal Server Error"),
    )
))]
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

/// Get authorized subjects
///
/// Allows obtaining the list of subjects that have been pre-authorized by the node, as well as the identifiers of the nodes from which to obtain them.
///
/// # Parameters
///
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
/// * `Query(parameters): Query<PaginatorFromString>` - The pagination parameters for the request.
///
/// # Returns
///
/// * `Result<Json<Vec<PreauthorizedSubjectsResponse>>, Errors>` - A list of pre-authorized subjects in JSON format or an error if the request fails.
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/allowed-subjects",
    operation_id = "Get Allowed Subject Data",
    tag = "Others",
    params(
        ("from" = Option<String>, Query, description = "Id of initial subject"),
        ("quantity" = Option<i64>, Query, description = "Quantity of subjects requested")
    ),
    responses(
        (status = 200, description = "Subject Data successfully retrieved", body = [PreauthorizedSubjectsResponse],
        example = json!(
            [
                {
                    "subject_id": "JKZgYhPjQdWNWWwkac0wSwqLKoOJsT0QimJmj6zjimWc",
                    "providers": []
                }
            ]
        )),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    )
))]
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

/// Set subject as preauthorized
///
/// Allows a subject to be established as pre-qualified. It can also be used to specify from which nodes in the network the resource should be obtained.
///
/// # Parameters
///
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject.
/// * `Json(authorize_subject): Json<AuthorizeSubject>` - The authorization details in JSON format.
///
/// # Returns
///
/// * `Result<Json<String>, Errors>` - The result of the pre-authorization as a JSON string or an error if the request fails.
#[cfg_attr(feature = "doc",  utoipa::path(
    put,
    path = "/allowed-subjects/{subject-id}",
    operation_id = "Put Allowed Subject Data",
    tag = "Others",
    params(
        ("subject-id" = String, Path, description = "Subject's unique id")
    ),
    request_body(content = AuthorizeSubject, content_type = "application/json", description = "Vote of the user for an existing request",
    example = json!(
        {
            "providers": []
        }
    )),
    responses(
        (status = 200, description = "Subject Data successfully created", body = String,
        example = json!("OK")),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
))]
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

/// Generate keys
///
/// Generate keys to create events.
///
/// # Parameters
///
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
/// * `Query(parameters): Query<NodeKeys>` - The query parameters for the request.
///
/// # Returns
///
/// * `Result<Json<String>, Errors>` - The generated keys as a JSON string or an error if the request fails.
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/generate-keys",
    operation_id = "Generate Keys",
    tag = "Others",
    params(
        ("algorithm" = Option<String>, Query, description = "Type of algorithm to use (possibilities: Ed25519, Secp256k1)")
    ),
    responses (
        (status = 200, description = "Public Key", body = String,
        example = json!(
            "E5R_R6sFSR28gWnLiU8f8pgOih_VguHhRXWbepuAJHGQ"
        )),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error")
    )
))]
async fn register_keys(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Query(parameters): Query<NodeKeys>,
) -> Result<Json<String>, Errors> {
    match kore_api.register_keys(parameters).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

/// Get subjects
///
/// Allows obtaining the list of subjects known by the node with pagination.
/// It can also be used to obtain only the governances and all subjects belonging to a specific governance.
///
/// # Parameters
///
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
/// * `Query(parameters): Query<NodeSubjects>` - The query parameters for the request.
///
/// # Returns
///
/// * `Result<Json<Vec<NodeSubjectData>>, Errors>` - A list of subjects in JSON format or an error if the request fails.
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/subjects",
    tag = "Subjects",
    operation_id = "Get All Subjects Data",
    params(
        ("subject_type" = Option<String>, Query, description = "Type of subjects requested (possibilities: all, governances)"),
        ("governanceid" = Option<String>, Query, description = "Governance id of subjects requested"),
        ("from" = Option<String>, Query, description = "Identifier of the initial subject to be considered in pagination"),
        ("quantity" = Option<isize>, Query, description = "Quantity of subjects requested")
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = [NodeSubjectData],
        example = json!(
            [
                {
                    "subject_id": "JEwuT__FAzdnXYY2Sg5BIZeCjjNnVFIuzHzGRtauykY8",
                    "governance_id": "",
                    "sn": 0,
                    "public_key": "EJ0irrPzgmZzawS6nYAoNOfjYsH9cjIDJPeRO4Hc5vmY",
                    "namespace": "",
                    "name": "tutorial",
                    "schema_id": "governance",
                    "owner": "EwkWURnRVk-lUEjF0cowczxYkz8DpbhLfo3UMSZE00LE",
                    "creator": "EwkWURnRVk-lUEjF0cowczxYkz8DpbhLfo3UMSZE00LE",
                    "properties": {
                    "members": [],
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
                        }
                    ],
                    "schemas": []
                    },
                    "active": true
                }
            ]
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
))]
async fn get_subjects(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Query(parameters): Query<NodeSubjects>,
) -> Result<Json<Vec<NodeSubjectData>>, Errors> {
    match kore_api.get_subjects(parameters).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}
/// Get subject by subject-id
///
/// Allows obtaining a specific subject by its identifier.
///
/// # Parameters
///
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject.
///
/// # Returns
///
/// * `Result<Json<NodeSubjectData>, Errors>` - The subject data in JSON format or an error if the request fails.
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/subjects/{subject-id}",
    operation_id = "Get Subject Data",
    tag = "Subjects",
    params(
        ("subject-id" = String, Path, description = "Subject's unique id")
    ),
    responses(
        (status = 200, description = "Subject Data successfully retrieved", body = NodeSubjectData,
        example = json!(
            {
                "subject_id": "JEwuT__FAzdnXYY2Sg5BIZeCjjNnVFIuzHzGRtauykY8",
                "governance_id": "",
                "sn": 0,
                "public_key": "EJ0irrPzgmZzawS6nYAoNOfjYsH9cjIDJPeRO4Hc5vmY",
                "namespace": "",
                "name": "tutorial",
                "schema_id": "governance",
                "owner": "EwkWURnRVk-lUEjF0cowczxYkz8DpbhLfo3UMSZE00LE",
                "creator": "EwkWURnRVk-lUEjF0cowczxYkz8DpbhLfo3UMSZE00LE",
                "properties": {
                  "members": [],
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
                    }
                  ],
                  "schemas": []
                },
                "active": true
              }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
))]
async fn get_subject(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Path(subject_id): Path<String>,
) -> Result<Json<NodeSubjectData>, Errors> {
    match kore_api.get_subject(&subject_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

/// Get validation proof
///
/// Allows obtaining the validation test of the last event for a specified subject.
///
/// # Parameters
///
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject.
///
/// # Returns
///
/// * `Result<Json<NodeProof>, Errors>` - The validation proof in JSON format or an error if the request fails.
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/subjects/{subject-id}/validation",
    operation_id = "Get Validation Proof",
    tag = "Subjects",
    params(
        ("subject-id" = String, Path, description = "Subject's unique id"),
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = NodeProof,
        example = json!(
            {
                "proof": {
                  "subject_id": "JEwuT__FAzdnXYY2Sg5BIZeCjjNnVFIuzHzGRtauykY8",
                  "schema_id": "governance",
                  "namespace": "",
                  "name": "tutorial",
                  "subject_public_key": "EJ0irrPzgmZzawS6nYAoNOfjYsH9cjIDJPeRO4Hc5vmY",
                  "governance_id": "",
                  "genesis_governance_version": 0,
                  "sn": 0,
                  "prev_event_hash": "",
                  "event_hash": "J40VKacq_EfrwIFZJTOEWjSm3RajiuG7T3l8YC5YCemM",
                  "governance_version": 0
                },
                "signatures": [
                  {
                    "signer": "EwkWURnRVk-lUEjF0cowczxYkz8DpbhLfo3UMSZE00LE",
                    "timestamp": 17182688,
                    "value": "SEMaLSVuDTpuPZ6ImO99R2jNKpPG4IonEhafmg9rnj3xIUOWauOZK-ZZlnjaYOPQwsKmJ8ff3n_EPJPs-l2S_5DA",
                    "content_hash": "JgGy23Fso8KgrLKG8BImiWeD9AHZYWL4kjIRkbNZ7fqU"
                  }
                ]
              }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
))]
async fn get_validation_proof(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Path(subject_id): Path<String>,
) -> Result<Json<NodeProof>, Errors> {
    match kore_api.get_validation_proof(&subject_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

/// Get events of a subject
///
/// Allows obtaining specific events of a subject by its identifier.
///
/// # Parameters
///
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
/// * `Path(subject_id): Path<String>` - The identifier of the subject.
/// * `Query(parameters): Query<PaginatorFromNumber>` - The pagination parameters for the request.
///
/// # Returns
///
/// * `Result<Json<Vec<NodeSigned<EventContentResponse>>>, Errors>` - A list of events in JSON format or an error if the request fails.
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/subjects/{subject-id}/events",
    operation_id = "Get Subject Events",
    tag = "Subjects",
    params(
        ("subject-id" = String, Path, description = "Subject's unique id"),
    ),
    responses(
        (status = 200, description = "Subject Data successfully retrieved", body = [NodeSignedEventContentResponse],
        example = json!(
                    [
                    {
                        "subject_id": "JEwuT__FAzdnXYY2Sg5BIZeCjjNnVFIuzHzGRtauykY8",
                        "event_request": {
                        "Create": {
                            "governance_id": "",
                            "schema_id": "governance",
                            "namespace": "",
                            "name": "tutorial",
                            "public_key": "EJ0irrPzgmZzawS6nYAoNOfjYsH9cjIDJPeRO4Hc5vmY"
                        },
                        "signature": {
                            "signer": "EwkWURnRVk-lUEjF0cowczxYkz8DpbhLfo3UMSZE00LE",
                            "timestamp": 1718268,
                            "value": "SEOEf5NFEFX_Ylf2dw0MG7y5Ckem4uSvr7YN7sdJWRF2s-OeNMmtPcP0d3QmYNqVlTUPixNrv5woQDuT19UWqlDg",
                            "content_hash": "JWLeRGGAJM_1DcpNVc888lMlgZCgqKplV8CZ-tBLH2p8"
                        }
                        },
                        "gov_version": 0,
                        "sn": 0,
                        "patch": [
                        {
                            "op": "add",
                            "path": "/members",
                            "value": []
                        },
                        {
                            "op": "add",
                            "path": "/policies",
                            "value": [
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
                            ]
                        },
                        {
                            "op": "add",
                            "path": "/roles",
                            "value": [
                            {
                                "namespace": "",
                                "role": "WITNESS",
                                "schema": {
                                "ID": "governance"
                                },
                                "who": "MEMBERS"
                            }
                            ]
                        },
                        {
                            "op": "add",
                            "path": "/schemas",
                            "value": []
                        }
                        ],
                        "state_hash": "JZrjWdX_tP5Q_9SDcW5bS9RbvN6SuZ62cxvjn3XcolLI",
                        "eval_success": true,
                        "appr_required": false,
                        "approved": true,
                        "hash_prev_event": "",
                        "evaluators": [],
                        "approvers": [],
                        "signature": {
                        "signer": "EJ0irrPzgmZzawS6nYAoNOfjYsH9cjIDJPeRO4Hc5vmY",
                        "timestamp": 1718268,
                        "value": "SER7GL9A9nSAMkmtySPgQpU2CLR2lHAJdtxmIe2wOc0ohWsN9BtIv7qXjgcWwi-fF5VXVqnjUHj0fy3CKKavtGCw",
                        "content_hash": "JARzRLhapGTQxEWHc9i9taSIXa0d6zuDNe-M-3MFVTSc"
                        }
                    }
                    ]
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
))]
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

/// Get an event from a subject
///
/// Allows obtaining a specific event from a subject.
///
/// # Parameters
///
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
/// * `Path((subject_id, sn)): Path<(String, u64)>` - The subject identifier and the sequence number of the event.
///
/// # Returns
///
/// * `Result<Json<NodeSigned<EventContentResponse>>, Errors>` - The requested event in JSON format or an error if the request fails.
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/subjects/{subject-id}/events/{sn}",
    operation_id = "Get Event",
    tag = "Subjects",
    params(
        ("subject-id" = String, Path, description = "Subject's unique id"),
        ("sn" = u64, Path, description = "Event sn"),
    ),
    responses(
        (status = 200, description = "Subjects Data successfully retrieved", body = NodeSignedEventContentResponse,
        example = json!(
            {
                "subject_id": "JEwuT__FAzdnXYY2Sg5BIZeCjjNnVFIuzHzGRtauykY8",
                "event_request": {
                  "Create": {
                    "governance_id": "",
                    "schema_id": "governance",
                    "namespace": "",
                    "name": "tutorial",
                    "public_key": "EJ0irrPzgmZzawS6nYAoNOfjYsH9cjIDJPeRO4Hc5vmY"
                  },
                  "signature": {
                    "signer": "EwkWURnRVk-lUEjF0cowczxYkz8DpbhLfo3UMSZE00LE",
                    "timestamp": 171826883,
                    "value": "SEOEf5NFEFX_Ylf2dw0MG7y5Ckem4uSvr7YN7sdJWRF2s-OeNMmtPcP0d3QmYNqVlTUPixNrv5woQDuT19UWqlDg",
                    "content_hash": "JWLeRGGAJM_1DcpNVc888lMlgZCgqKplV8CZ-tBLH2p8"
                  }
                },
                "gov_version": 0,
                "sn": 0,
                "patch": [
                  {
                    "op": "add",
                    "path": "/members",
                    "value": []
                  },
                  {
                    "op": "add",
                    "path": "/policies",
                    "value": [
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
                    ]
                  },
                  {
                    "op": "add",
                    "path": "/roles",
                    "value": [
                      {
                        "namespace": "",
                        "role": "WITNESS",
                        "schema": {
                          "ID": "governance"
                        },
                        "who": "MEMBERS"
                      }
                    ]
                  },
                  {
                    "op": "add",
                    "path": "/schemas",
                    "value": []
                  }
                ],
                "state_hash": "JZrjWdX_tP5Q_9SDcW5bS9RbvN6SuZ62cxvjn3XcolLI",
                "eval_success": true,
                "appr_required": false,
                "approved": true,
                "hash_prev_event": "",
                "evaluators": [],
                "approvers": [],
                "signature": {
                  "signer": "EJ0irrPzgmZzawS6nYAoNOfjYsH9cjIDJPeRO4Hc5vmY",
                  "timestamp": 171826883,
                  "value": "SER7GL9A9nSAMkmtySPgQpU2CLR2lHAJdtxmIe2wOc0ohWsN9BtIv7qXjgcWwi-fF5VXVqnjUHj0fy3CKKavtGCw",
                  "content_hash": "JARzRLhapGTQxEWHc9i9taSIXa0d6zuDNe-M-3MFVTSc"
                }
              }
        )),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error"),
    )
))]
async fn get_event_of_subject(
    Extension(kore_api): Extension<Arc<KoreApi>>,
    Path((subject_id, sn)): Path<(String, u64)>,
) -> Result<Json<NodeSigned<EventContentResponse>>, Errors> {
    match kore_api.get_event_of_subject(&subject_id, sn).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Errors::Kore(e.to_string())),
    }
}

/// Get Controller-id
///
/// Returns the controller-id (public key of the node).
///
/// # Parameters
///
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
///
/// # Returns
///
/// * `Json<String>` - The controller-id as a JSON string.
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/controller-id",
    operation_id = "Get Controller-id",
    tag = "Others",
    responses (
        (status = 200, description = "Controller-id", body = String,
        example = json!(
            "E5X1tJWs1EQbByLV_zndMF0ml-wSyxHqh0pINRETWMjA"
        )),
        (status = 500, description = "Internal Server Error")
    )
))]
async fn get_controller_id(Extension(kore_api): Extension<Arc<KoreApi>>) -> Json<String> {
    Json(kore_api.get_controller_id())
}

/// Get Peer-id
///
/// Returns the peer-id (unique identifier of the node in the network).
///
/// # Parameters
///
/// * `Extension(kore_api): Extension<Arc<KoreApi>>` - The Kore API extension wrapped in an `Arc`.
///
/// # Returns
///
/// * `Json<String>` - The peer-id as a JSON string.
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/peer-id",
    operation_id = "Get Peer-id",
    tag = "Others",
    responses (
        (status = 200, description = "Peer-id", body = String,
        example = json!(
            "12D3KooWRGCTbLUyz9JpchPER5NFSAQGPbrQufAPPXaLJhccsQes"
        )),
        (status = 500, description = "Internal Server Error")
    )
))]
async fn get_peer_id(Extension(kore_api): Extension<Arc<KoreApi>>) -> Json<String> {
    Json(kore_api.get_peer_id())
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
        .route("/controller-id", get(get_controller_id))
        .route("/peer-id", get(get_peer_id))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(ips_connects_state))
                .layer(axum::middleware::from_fn(limit_ip_request))
                .layer(Extension(kore_api)),
        );
    #[cfg(feature = "doc")]
    return Router::new().merge(routes).merge(
        RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/doc"),
    );
    #[cfg(not(feature = "doc"))]
    Router::new().merge(routes)
}
