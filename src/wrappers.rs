use kore_bridge::{RequestData as RequestDataBridge, GovsData as GovsDataBridge, RegisterData as RegisterDataBridge, RequestInfo as RequestInfoBridge, ApproveInfo as ApproveInfoBridge, ApprovalReqInfo as ApprovalReqInfoBridge, SignedInfo as SignedInfoBridge, FactInfo as FactInfoBridge, SignatureInfo as SignatureInfoBridge};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RequestData {
    pub request_id: String,
    pub subject_id: String,
}

impl From<RequestDataBridge> for RequestData {
    fn from(value: RequestDataBridge) -> Self {
        Self {
            request_id: value.request_id,
            subject_id: value.subject_id
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GovsData {
    pub governance_id: String,
    pub active: bool,
}

impl From<GovsDataBridge> for GovsData {
    fn from(value: GovsDataBridge) -> Self {
        Self { governance_id: value.governance_id, active: value.active }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterData {
    pub subject_id: String,
    pub schema: String,
    pub active: bool,
}

impl From<RegisterDataBridge> for RegisterData {
    fn from(value: RegisterDataBridge) -> Self {
        Self { subject_id: value.subject_id, schema: value.schema, active: value.active }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RequestInfo {
    status: String,
    version: u64,
    error: Option<String>,
}

impl From<RequestInfoBridge> for RequestInfo {
    fn from(value: RequestInfoBridge) -> Self {
        Self { status: value.status, version: value.version, error: value.error }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApproveInfo {
    pub state: String,
    pub request: ApprovalReqInfo,
}

impl From<ApproveInfoBridge> for ApproveInfo {
    fn from(value: ApproveInfoBridge) -> Self {
        Self { state: value.state, request: ApprovalReqInfo::from(value.request) }    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApprovalReqInfo {
        /// The signed event request.
    pub event_request: SignedInfo<FactInfo>,
        /// The sequence number of the event.
    pub sn: u64,
        /// The version of the governance contract.
    pub gov_version: u64,
        /// The patch to apply to the state.
    pub patch: Value,
        /// The hash of the state after applying the patch.
    pub state_hash: String,
        /// The hash of the previous event.
    pub hash_prev_event: String,
        /// The hash of the previous event.
    pub subject_id: String,
}

impl From<ApprovalReqInfoBridge> for ApprovalReqInfo {
    fn from(value: ApprovalReqInfoBridge) -> Self {
        Self { event_request: SignedInfo::from(value.event_request), sn: value.sn, gov_version: value.gov_version, patch: value.patch, state_hash: value.state_hash, hash_prev_event: value.hash_prev_event, subject_id: value.subject_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SignedInfo<T>
where
    T: Serialize + Clone,
{
    pub content: T,
    pub signature: SignatureInfo,
}

impl From<SignedInfoBridge<FactInfoBridge>> for SignedInfo<FactInfo> {
    fn from(value: SignedInfoBridge<FactInfoBridge>) -> Self {
        Self { content: FactInfo::from(value.content), signature: SignatureInfo::from(value.signature) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FactInfo {
    payload: Value,
    subject_id: String,
}

impl From<FactInfoBridge> for FactInfo {
    fn from(value: FactInfoBridge) -> Self {
        Self { payload: value.payload, subject_id: value.subject_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SignatureInfo {
    pub signer: String,
    pub timestamp: u64,
    pub content_hash: String,
    pub value: String,
}

impl From<SignatureInfoBridge> for SignatureInfo {
    fn from(value: SignatureInfoBridge) -> Self {
        Self { signer: value.signer, timestamp: value.timestamp, content_hash: value.content_hash, value: value.value }
    }
}