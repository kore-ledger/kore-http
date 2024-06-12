use utoipa::OpenApi;

use crate::doc::wrappers::*;
use crate::server::*;

#[derive(OpenApi)]
#[openapi(
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
        (name = "Approvals"),
        (name = "Requests"),
        (name = "Subjects"),
        (name = "Others"),
    )
)]
pub struct ApiDoc;
