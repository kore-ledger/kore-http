use utoipa::OpenApi;

use crate::doc::wrappers::*;
use crate::server::*;

/// Kore HTTP
///
/// This API provides interaction with Kore Ledger nodes using the HTTP protocol.
/// It allows sending and retrieving various types of requests and managing subjects.
/// The API is documented with OpenAPI for easy integration and use.
/// 
/// # Configuration
/// 
/// This client uses a single configuration variable, which is set through an environment variable.
/// Ensure that the environment variable is properly configured before using this API.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Kore HTTP",
        description = "This API provides interaction with Kore Ledger nodes using the HTTP protocol. It allows sending and retrieving various types of requests and managing subjects. The API is documented with OpenAPI for easy integration and use.",
        version = "1.0.0",
        contact(
            name = "Kore Information",
            url = "https://www.kore-ledger.net/",
            email = "info@kore-ledger.net"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    paths(
        send_event_request,
        get_event_request,
        get_event_request_state,
        get_approvals,
        get_approval_id,
        approval_request,
        get_all_allowed_subjects_and_providers,
        add_preauthorize_subject,
        register_keys,
        get_subjects,
        get_subject,
        get_validation_proof,
        get_events_of_subject,
        get_event_of_subject,
        get_controller_id,
        get_peer_id,
    ),
    components(
        schemas(
            NodeSignedEventRequest,
            NodeEventRequest,
            NodeEOLRequest,
            NodeTransferRequest,
            NodeFactRequest,
            NodeStartRequest,
            NodeSignature,
            AuthorizeSubject,
            EventContentResponse,
            NodeSignedNodeApprovalRequest,
            NodeSignedNodeApprovalResponse,
            NodeSignedNodeEventRequest,
            EventRequestResponse,
            NodeApprovalEntity,
            NodeApprovalRequest,
            NodeApprovalResponse,
            ApprovalState,
            NodeKoreRequestState,
            RequestState,
            NodeProof,
            NodeValidationProof,
            NodeSubjectData,
            PatchVote,
            PreauthorizedSubjectsResponse
        )
    ),
    tags(
        (name = "Approvals", description = "Endpoints related to request approvals."),
        (name = "Requests", description = "Endpoints for managing event requests."),
        (name = "Subjects", description = "Endpoints for managing subjects and their data."),
        (name = "Others", description = "Miscellaneous endpoints for node identification and configuration."),
    )
)]
pub struct ApiDoc;
