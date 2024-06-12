use core::str;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;


/// NodeSignedEventRequest wrapper to implement ToSchema.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct NodeSignedEventRequest {
    pub request: NodeEventRequest,
    pub signature: Option<NodeSignature>
}

#[derive(Serialize, Deserialize, ToSchema)]
pub enum NodeEventRequest {
    Create(NodeStartRequest),
    Fact(NodeFactRequest),
    Transfer(NodeTransferRequest),
    EOL(NodeEOLRequest),
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NodeEOLRequest {
    pub subject_id: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NodeTransferRequest {
    pub subject_id: String,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NodeFactRequest {
    pub subject_id: String,
    pub payload: Value,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NodeStartRequest {
    pub governance_id: String,
    pub schema_id: String,
    pub namespace: String,
    pub name: String,
    pub public_key: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NodeSignature {
    signer: String,
    timestamp: u64,
    value: String,
    content_hash: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AuthorizeSubject {
    pub providers: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EventContentResponse {
    pub subject_id: String,
    pub event_request: NodeSignedNodeEventRequest,
    pub gov_version: u64,
    /// Current sequence number of the subject
    pub sn: u64,
    /// Changes to be applied to the subject
    pub patch: Value,
    /// Hash of the state
    pub state_hash: String,
    /// Value specifying if the evaluation process has gone well
    pub eval_success: bool,
    /// Value specifying if approval is required
    pub appr_required: bool,
    /// Value specifying if it has been approved
    pub approved: bool,
    /// Previous event hash
    pub hash_prev_event: String,
    /// Signatures of the evaluators
    pub evaluators: Vec<NodeSignature>,
    /// Signatures of the approvers
    pub approvers: Vec<NodeSignature>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NodeSigned<T>
{
    /// Content
    #[serde(flatten)]
    pub content: T,
    /// Signature
    pub signature: NodeSignature,
}

#[derive(Serialize, Deserialize)]
pub struct NodeSignedNodeApprovalRequest(pub NodeSigned<NodeApprovalRequest>);

impl<'__s> utoipa::ToSchema<'__s> for NodeSignedNodeApprovalRequest {
    fn schema() -> (
        &'__s str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        let schema_event = NodeApprovalRequest::schema();
        let schema_signature = NodeSignature::schema();
        (
            "NodeSignedNodeApprovalRequest",
            utoipa::openapi::ObjectBuilder::new()
                .property(schema_event.0, schema_event.1)
                .property(schema_signature.0, schema_signature.1)
                .into(),
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct NodeSignedNodeApprovalResponse(pub NodeSigned<NodeApprovalResponse>);

impl<'__s> utoipa::ToSchema<'__s> for NodeSignedNodeApprovalResponse {
    fn schema() -> (
        &'__s str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        let schema_event = NodeApprovalResponse::schema();
        let schema_signature = NodeSignature::schema();
        (
            "NodeSignedNodeApprovalResponse",
            utoipa::openapi::ObjectBuilder::new()
                .property(schema_event.0, schema_event.1)
                .property(schema_signature.0, schema_signature.1)
                .into(),
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct NodeSignedNodeEventRequest(pub NodeSigned<NodeEventRequest>);

impl<'__s> utoipa::ToSchema<'__s> for NodeSignedNodeEventRequest {
    fn schema() -> (
        &'__s str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        let schema_event = NodeEventRequest::schema();
        let schema_signature = NodeSignature::schema();
        (
            "NodeSignedNodeEventRequest",
            utoipa::openapi::ObjectBuilder::new()
                .property(schema_event.0, schema_event.1)
                .property(schema_signature.0, schema_signature.1)
                .into(),
        )
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EventRequestResponse {
    pub request_id: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NodeApprovalEntity {
    pub id: String,
    pub request: NodeSignedNodeApprovalRequest,
    pub reponse: Option<NodeSignedNodeApprovalResponse>,
    pub state: ApprovalState,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NodeApprovalRequest {
    pub event_request: NodeSignedEventRequest,
    pub sn: u64,
    pub gov_version: u64,
    pub patch: Value,
    pub state_hash: String,
    pub hash_prev_event: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NodeApprovalResponse {
    pub appr_req_hash: String,
    pub approved: bool,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub enum ApprovalState {
    Pending,
    RespondedAccepted,
    RespondedRejected,
    Obsolete,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NodeKoreRequestState {
    pub id: String,
    pub subject_id: Option<String>,
    pub sn: Option<u64>,
    pub state: RequestState,
    pub success: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub enum RequestState {
    Finished,
    Error,
    Processing,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NodeProof {
    pub proof: NodeValidationProof,
    pub signatures: Vec<NodeSignature>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NodeValidationProof {
    pub subject_id: String,
    pub schema_id: String,
    pub namespace: String,
    pub name: String,
    pub subject_public_key: String,
    pub governance_id: String,
    pub genesis_governance_version: u64,
    pub sn: u64,
    pub prev_event_hash: String,
    pub event_hash: String,
    pub governance_version: u64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NodeSubjectData {

    pub subject_id: String,
    pub governance_id: String,
    pub sn: u64,
    pub public_key: String,
    pub namespace: String,
    pub name: String,
    pub schema_id: String,
    pub owner: String,
    pub creator: String,
    pub properties: Value,
    pub active: bool,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub enum PatchVote {
    RespondedAccepted,
    RespondedRejected,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PreauthorizedSubjectsResponse {
    pub subject_id: String,
    pub providers: Vec<String>,
}