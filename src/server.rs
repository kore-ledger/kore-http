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

use crate::{
    common::IPSMaxConnectState, error::Errors,
    middleware::middlewares::limit_ip_request,
};
#[cfg(feature = "doc")]
use crate::doc::utoipa::ApiDoc;
#[cfg(feature = "doc")]
use utoipa::OpenApi;
#[cfg(feature = "doc")]
use utoipa_swagger_ui::SwaggerUi;

/// Send event request
///
/// Allows to send an event request for a subject to the Kore node.
/// These requests can be of any type of event (done, creation, transfer and end of life).
/// In case of external invocation, the requests can be signed.

#[cfg_attr(feature = "doc", utoipa::path(
    post,
    path = "/event-requests",
    tag = "Requests",
    operation_id = "Create Event Request",
    request_body = NodeSignedEventRequest,
    responses(
        (status = 201, description = "Request Created Successfully", body = EventRequestResponse,
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
/// Allows to obtain an event request by its identifier
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
                "Fact": {
                    "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                    "payload": {
                        "Patch": {
                            "data": [
                                {
                                    "op": "add",
                                    "path": "/members/0",
                                    "value": {
                                        "id": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                        "name": "WPO"
                                    }
                                }
                            ]
                        }
                    }
                },
                "signature": {
                    "signer": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                    "timestamp": 1688643580,
                    "value": "SE4yS1Q1Smhm3Az3r6WNFKAGd2Us69vyUA3j5q_riE6MICh_Ub2fSLxNS3Nn-g_CpppvABq6s_c8dF5kbmUir4Ag"
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
/// Allows to obtain the status of an event request by its identifier.
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
/// Allows to obtain the list of requests for approvals received by the node.
/// It can also be used, by means of the "status" parameter, to list the requests pending approval.
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/approval-requests",
    operation_id = "Get all Approvals Request Data",
    tag = "Approvals",
    params(
        ("status" = Option<String>, Query, description = "Approval's status (possibilities: pending, obsolete, responded)"),
        ("from" = Option<String>, Query, description = "Id of initial approval"),
        ("quantity" = Option<i64>, Query, description = "Quantity of approvals requested"
    )),
    responses(
        (status = 200, description = "Approvals Data successfully retrieved", body = [NodeApprovalEntity],
        example = json!(
            [
                {
                    "id": "J5dfpH-ahrqSo-od4jyZkubyO-XWFJSQ9maK73jKI4Ao",
                    "request": {
                        "event_request": {
                            "Fact": {
                                "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                                "payload": {
                                    "Patch": {
                                        "data": [
                                            {
                                                "op": "add",
                                                "path": "/members/0",
                                                "value": {
                                                    "id": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                                    "name": "WPO"
                                                }
                                            }
                                        ]
                                    }
                                }
                            },
                            "signature": {
                                "signer": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                "timestamp": 168864358,
                                "value": "SE4yS1Q1Smhm3Az3r6WNFKAGd2Us69vyUA3j5q_riE6MICh_Ub2fSLxNS3Nn-g_CpppvABq6s_c8dF5kbmUir4Ag"
                            }
                        },
                        "sn": 1,
                        "gov_version": 0,
                        "patch": [
                            {
                                "op": "add",
                                "path": "/members/0",
                                "value": {
                                    "id": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                    "name": "WPO"
                                }
                            }
                        ],
                        "state_hash": "JbDVCZxkDkZ5gLCc7Ge5X75pHHf8dA7_s8UynsnzG5o8",
                        "hash_prev_event": "JLic8SLrT7tJxA9B3aLaaKaIEuV7Wouo2ogHCid6O4g8",
                        "signature": {
                            "signer": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                            "timestamp": 168864358,
                            "value": "SEFyfXR6kE04gGdCtXZN-So6nNJAAe1qwnTkl0UuoFpCEEuQhwkZND77o1Y9OVuVus8mgGtyAdTi-A7_0MkDKgBw"
                        }
                    },
                    "reponse": null,
                    "state": "Pending"
                }
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
/// Allows you to obtain a request for approval by its identifier.
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
                "id": "J5dfpH-ahrqSo-od4jyZkubyO-XWFJSQ9maK73jKI4Ao",
                "request": {
                    "event_request": {
                        "Fact": {
                            "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                            "payload": {
                                "Patch": {
                                    "data": [
                                        {
                                            "op": "add",
                                            "path": "/members/0",
                                            "value": {
                                                "id": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                                "name": "WPO"
                                            }
                                        }
                                    ]
                                }
                            }
                        },
                        "signature": {
                            "signer": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                            "timestamp": 1688643580,
                            "value": "SE4yS1Q1Smhm3Az3r6WNFKAGd2Us69vyUA3j5q_riE6MICh_Ub2fSLxNS3Nn-g_CpppvABq6s_c8dF5kbmUir4Ag"
                        }
                    },
                    "sn": 1,
                    "gov_version": 0,
                    "patch": [
                        {
                            "op": "add",
                            "path": "/members/0",
                            "value": {
                                "id": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                "name": "WPO"
                            }
                        }
                    ],
                    "state_hash": "JbDVCZxkDkZ5gLCc7Ge5X75pHHf8dA7_s8UynsnzG5o8",
                    "hash_prev_event": "JLic8SLrT7tJxA9B3aLaaKaIEuV7Wouo2ogHCid6O4g8",
                    "signature": {
                        "signer": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                        "timestamp": 1688643580,
                        "value": "SEFyfXR6kE04gGdCtXZN-So6nNJAAe1qwnTkl0UuoFpCEEuQhwkZND77o1Y9OVuVus8mgGtyAdTi-A7_0MkDKgBw"
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
/// Allows you to issue an affirmative or negative approval for a previously received request.
#[cfg_attr(feature = "doc",  utoipa::path(
    patch,
    path = "/approval-requests/{id}",
    operation_id = "Set your Aprroval for a request",
    tag = "Approvals",
    request_body(content = PatchVote, content_type = "application/json", description = "Vote of the user for an existing request",
    example = json!(
        {
            "state": "RespondedAccepted"
        }
    )),
    params(
        ("id" = String, Path, description = "Approval's unique id"),
    ),
    responses(
        (status = 204, description = "Request successfully voted", body = NodeApprovalEntity,
        example = json!(
            {
                "id": "J5dfpH-ahrqSo-od4jyZkubyO-XWFJSQ9maK73jKI4Ao",
                "request": {
                    "event_request": {
                        "Fact": {
                            "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                            "payload": {
                                "Patch": {
                                    "data": [
                                        {
                                            "op": "add",
                                            "path": "/members/0",
                                            "value": {
                                                "id": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                                "name": "WPO"
                                            }
                                        }
                                    ]
                                }
                            }
                        },
                        "signature": {
                            "signer": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                            "timestamp": 1688643580,
                            "value": "SE4yS1Q1Smhm3Az3r6WNFKAGd2Us69vyUA3j5q_riE6MICh_Ub2fSLxNS3Nn-g_CpppvABq6s_c8dF5kbmUir4Ag"
                        }
                    },
                    "sn": 1,
                    "gov_version": 0,
                    "patch": [
                        {
                            "op": "add",
                            "path": "/members/0",
                            "value": {
                                "id": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                                "name": "WPO"
                            }
                        }
                    ],
                    "state_hash": "JbDVCZxkDkZ5gLCc7Ge5X75pHHf8dA7_s8UynsnzG5o8",
                    "hash_prev_event": "JLic8SLrT7tJxA9B3aLaaKaIEuV7Wouo2ogHCid6O4g8",
                    "signature": {
                        "signer": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                        "timestamp": 1688643580,
                        "value": "SEFyfXR6kE04gGdCtXZN-So6nNJAAe1qwnTkl0UuoFpCEEuQhwkZND77o1Y9OVuVus8mgGtyAdTi-A7_0MkDKgBw"
                    }
                },
                "reponse": {
                    "appr_req_hash": "J5dfpH-ahrqSo-od4jyZkubyO-XWFJSQ9maK73jKI4Ao",
                    "approved": true,
                    "signature": {
                        "signer": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                        "timestamp": 168864361,
                        "value": "SERUEr362pHPIcORhUnYPxnW1A_jW675_yphYIQIKaO6wytdh7xwwNTXHW6Q1fs9F6ag8VpTy2DM_5ppRT7irFDg"
                    }
                },
                "state": "Responded"
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
/// Allows to obtain the list of subjects that have been pre-authorized by the node, as well as the identifiers of the nodes from which to obtain them.
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
/// Generate keys to create events
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
/// Allows to obtain, with pagination, the list of subjects known by the node.
/// It can also be used to obtain exclusively the governances and all the subjects belonging to a specific one.
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
                    "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                    "governance_id": "",
                    "sn": 0,
                    "public_key": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                    "namespace": "",
                    "name": "Wine_Producers_Organization",
                    "schema_id": "governance",
                    "owner": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                    "creator": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
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
                        "roles": [],
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
/// Allows to obtain a specific subject by means of its identifier
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
                "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                "governance_id": "",
                "sn": 0,
                "public_key": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                "namespace": "",
                "name": "Wine_Producers_Organization",
                "schema_id": "governance",
                "owner": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                "creator": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
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
                    "roles": [],
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
/// Allows to obtain the validation test of the last event for a specified subject.
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
                    "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                    "schema_id": "governance",
                    "namespace": "",
                    "name": "Wine_Producers_Organization",
                    "subject_public_key": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                    "governance_id": "",
                    "genesis_governance_version": 0,
                    "sn": 0,
                    "prev_event_hash": "",
                    "event_hash": "JLic8SLrT7tJxA9B3aLaaKaIEuV7Wouo2ogHCid6O4g8",
                    "governance_version": 0
                },
                "signatures": [
                    {
                        "signer": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                        "timestamp": 1688643031,
                        "value": "SEF3qN1uKIgNfnK6YlgU7rlCvDCNHhl_tdcRBvQRyGShR8oOOw5tVk8_OUNlyaJV_HsrISeX8jAf4L3diodRZ_Dg"
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
/// Get subject by ID
/// Allows to obtain a specific subject by means of its identifier
#[cfg_attr(feature = "doc",  utoipa::path(
    get,
    path = "/subjects/{subject-id}/events",
    operation_id = "Get Subject Events",
    tag = "Subjects",
    params(
        ("subject-id" = String, Path, description = "Subject's unique id"),
    ),
    responses(
        (status = 200, description = "Subject Data successfully retrieved", body = NodeSignedEventContentResponse,
        example = json!(
            {
                "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                "governance_id": "",
                "sn": 0,
                "public_key": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                "namespace": "",
                "name": "Wine_Producers_Organization",
                "schema_id": "governance",
                "owner": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                "creator": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
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
                    "roles": [],
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
/// Allows to obtain a specific event from a subject
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
                "subject_id": "JoifaSpfenD2bEPeBLvUTWh30brm4tKcvdW8exQnkGoQ",
                "event_request": {
                    "Create": {
                        "governance_id": "",
                        "schema_id": "governance",
                        "namespace": "",
                        "name": "Wine_Producers_Organization",
                        "public_key": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs"
                    },
                    "signature": {
                        "signer": "EbwR0yYrCYpTzlN5i5GX_MtAbKRw5y2euv3TqiTgwggs",
                        "timestamp": 168864303,
                        "value": "SE-tHjb3eWcMvVIYuSBPn0EW4Q5mQs2uswS5HLl0GB0iYVEc5jcOWD78ZHRL8VlO0mtxv9KWt2EI9R9Id2Z5o8CA"
                    }
                },
                "sn": 0,
                "gov_version": 0,
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
                        "value": []
                    },
                    {
                        "op": "add",
                        "path": "/schemas",
                        "value": []
                    }
                ],
                "state_hash": "JVKr8BAEs1DhpNjMZf4525IYps2Gu6m7ZBmuaNBoM_Qk",
                "eval_success": true,
                "appr_required": false,
                "approved": true,
                "hash_prev_event": "",
                "evaluators": [],
                "approvers": [],
                "signature": {
                    "signer": "E0gaiDcPRVmYLUGbseHmBk2_2H-FAlSgaO6ZMOXhh4Gs",
                    "timestamp": 168864303,
                    "value": "SEnTz4Nw-rX6y00yNF01o__AwyWxyG1s669AetXCfrnxCTSyf67xv8AsnccTOe4fFm-2ZIeRjubdf5FTQHZAd7BQ"
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
/// Returns the controller-id(public key of the node).
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
/// Returns the peer-id(unique identifier of the node in the network).
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
    return Router::new().merge(routes).merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));
    #[cfg(not(feature = "doc"))]
    Router::new().merge(routes)
}
