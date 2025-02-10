use std::collections::HashSet;

use kore_bridge::{
    ApprovalReqInfo as ApprovalReqInfoBridge, ApproveInfo as ApproveInfoBridge, ConfirmRequestInfo as ConfirmRequestInfoBridge, CreateRequestInfo as CreateRequestInfoBridge, EOLRequestInfo as EOLRequestInfoBridge, EventInfo as EventInfoBridge, EventRequestInfo as EventRequestInfoBridge, FactInfo as FactInfoBridge, FactRequestInfo as FactRequestInfoBridge, GovsData as GovsDataBridge, Namespace as NamespaceBridge, Paginator as PaginatorBridge, PaginatorEvents as PaginatorEventsBridge, ProtocolsError as ProtocolsErrorBridge, ProtocolsSignaturesInfo as ProtocolsSignaturesInfoBridge, RegisterData as RegisterDataBridge, RejectRequestInfo as RejectRequestInfoBridge, RequestData as RequestDataBridge, RequestInfo as RequestInfoBridge, SignatureInfo as SignatureInfoBridge, SignaturesInfo as SignaturesInfoBridge, SignedInfo as SignedInfoBridge, SubjectInfo as SubjectInfoBridge, TimeOutResponseInfo as TimeOutResponseInfoBridge, TransferRequestInfo as TransferRequestInfoBridge
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatorEvents {
    pub paginator: Paginator,
    pub events: Vec<EventInfo>,
}

impl From<PaginatorEventsBridge> for PaginatorEvents {
    fn from(value: PaginatorEventsBridge) -> Self {
        Self { paginator: Paginator::from(value.paginator), events: value.events.iter().map(|x| EventInfo::from(x.clone())).collect() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventInfo {
    pub subject_id: String,
    pub sn: u64,
    pub patch: Option<Value>,
    pub error: Option<ProtocolsError>,
    pub event_req: EventRequestInfo,
    pub succes: bool,
}

impl From<EventInfoBridge> for EventInfo {
    fn from(value: EventInfoBridge) -> Self {
        let error = if let Some(error) = value.error {
            Some(ProtocolsError::from(error))
        } else {
            None
        };

        Self { subject_id: value.subject_id, sn: value.sn, patch: value.patch, error, event_req: EventRequestInfo::from(value.event_req), succes: value.succes }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Paginator {
    pub pages: u64,
    pub next: Option<u64>,
    pub prev: Option<u64>,
}

impl From<PaginatorBridge> for Paginator {
    fn from(value: PaginatorBridge) -> Self {
        Self { pages: value.pages, next: value.next, prev: value.prev }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProtocolsError {
    pub evaluation: Option<String>,
    pub validation: Option<String>,
}

impl From<ProtocolsErrorBridge> for ProtocolsError {
    fn from(value: ProtocolsErrorBridge) -> Self {
        Self { evaluation: value.evaluation, validation: value.validation }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum EventRequestInfo {
    Create(CreateRequestInfo),
    Fact(FactRequestInfo),
    Transfer(TransferRequestInfo),
    Confirm(ConfirmRequestInfo),
    Reject(RejectRequestInfo),
    EOL(EOLRequestInfo),
}

impl From<EventRequestInfoBridge> for EventRequestInfo {
    fn from(value: EventRequestInfoBridge) -> Self {
        match value {
            EventRequestInfoBridge::Create(create_request_info) => Self::Create(CreateRequestInfo::from(create_request_info)),
            EventRequestInfoBridge::Fact(fact_request_info) => Self::Fact(FactRequestInfo::from(fact_request_info)),
            EventRequestInfoBridge::Transfer(transfer_request_info) => Self::Transfer(TransferRequestInfo::from(transfer_request_info)),
            EventRequestInfoBridge::Confirm(confirm_request_info) => Self::Confirm(ConfirmRequestInfo::from(confirm_request_info)),
            EventRequestInfoBridge::Reject(reject_request_info) => Self::Reject(RejectRequestInfo::from(reject_request_info)),
            EventRequestInfoBridge::EOL(eolrequest_info) => Self::EOL(EOLRequestInfo::from(eolrequest_info)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRequestInfo {
    pub governance_id: String,
    pub schema_id: String,
    pub namespace: Namespace,
}

impl From<CreateRequestInfoBridge> for CreateRequestInfo {
    fn from(value: CreateRequestInfoBridge) -> Self {
        Self { governance_id: value.governance_id, schema_id: value.schema_id, namespace: Namespace::from(value.namespace) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TransferRequestInfo {
    pub subject_id: String,
    pub new_owner: String,
}

impl From<TransferRequestInfoBridge> for TransferRequestInfo {
    fn from(value: TransferRequestInfoBridge) -> Self {
        Self { subject_id: value.subject_id, new_owner: value.new_owner }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ConfirmRequestInfo {
    pub subject_id: String,
    pub name_old_owner: Option<String>,}

impl From<ConfirmRequestInfoBridge> for ConfirmRequestInfo {
    fn from(value: ConfirmRequestInfoBridge) -> Self {
        Self { subject_id: value.subject_id, name_old_owner: value.name_old_owner }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RejectRequestInfo {
    pub subject_id: String,
}

impl From<RejectRequestInfoBridge> for RejectRequestInfo {
    fn from(value: RejectRequestInfoBridge) -> Self {
        Self { subject_id: value.subject_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EOLRequestInfo {
    pub subject_id: String,
}

impl From<EOLRequestInfoBridge> for EOLRequestInfo {
    fn from(value: EOLRequestInfoBridge) -> Self {
        Self { subject_id: value.subject_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FactRequestInfo {
    pub subject_id: String,
    pub payload: Value,
}

impl From<FactRequestInfoBridge> for FactRequestInfo {
    fn from(value: FactRequestInfoBridge) -> Self {
        Self { subject_id: value.subject_id, payload: value.payload }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Namespace(Vec<String>);

impl From<NamespaceBridge> for Namespace {
    fn from(value: NamespaceBridge) -> Self {
        Namespace::from(value.to_string())
    }
}


impl From<&str> for Namespace {
    fn from(str: &str) -> Self {
        let tokens: Vec<String> = str
            .split('.')
            .filter(|x| !x.trim().is_empty())
            .map(|s| s.to_string())
            .collect();

        Namespace(tokens)
    }
}

impl From<String> for Namespace {
    fn from(str: String) -> Self {
        Namespace::from(str.as_str())
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RequestData {
    pub request_id: String,
    pub subject_id: String,
}

impl From<RequestDataBridge> for RequestData {
    fn from(value: RequestDataBridge) -> Self {
        Self {
            request_id: value.request_id,
            subject_id: value.subject_id,
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
        Self {
            governance_id: value.governance_id,
            active: value.active,
        }
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
        Self {
            subject_id: value.subject_id,
            schema: value.schema,
            active: value.active,
        }
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
        Self {
            status: value.status,
            version: value.version,
            error: value.error,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApproveInfo {
    pub state: String,
    pub request: ApprovalReqInfo,
}

impl From<ApproveInfoBridge> for ApproveInfo {
    fn from(value: ApproveInfoBridge) -> Self {
        Self {
            state: value.state,
            request: ApprovalReqInfo::from(value.request),
        }
    }
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
        Self {
            event_request: SignedInfo::from(value.event_request),
            sn: value.sn,
            gov_version: value.gov_version,
            patch: value.patch,
            state_hash: value.state_hash,
            hash_prev_event: value.hash_prev_event,
            subject_id: value.subject_id,
        }
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
        Self {
            content: FactInfo::from(value.content),
            signature: SignatureInfo::from(value.signature),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FactInfo {
    payload: Value,
    subject_id: String,
}

impl From<FactInfoBridge> for FactInfo {
    fn from(value: FactInfoBridge) -> Self {
        Self {
            payload: value.payload,
            subject_id: value.subject_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq, Hash)]
pub struct SignatureInfo {
    pub signer: String,
    pub timestamp: u64,
    pub content_hash: String,
    pub value: String,
}

impl From<SignatureInfoBridge> for SignatureInfo {
    fn from(value: SignatureInfoBridge) -> Self {
        Self {
            signer: value.signer,
            timestamp: value.timestamp,
            content_hash: value.content_hash,
            value: value.value,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SubjectInfo {
    pub subject_id: String,
    pub governance_id: String,
    pub genesis_gov_version: u64,
    pub namespace: String,
    pub schema_id: String,
    pub owner: String,
    pub creator: String,
    pub active: bool,
    pub sn: u64,
    pub properties: Value,
    pub new_owner: Option<String>
}

impl From<SubjectInfoBridge> for SubjectInfo {
    fn from(value: SubjectInfoBridge) -> Self {
        Self {
            subject_id: value.subject_id,
            governance_id: value.governance_id,
            genesis_gov_version: value.genesis_gov_version,
            namespace: value.namespace,
            schema_id: value.schema_id,
            owner: value.owner,
            creator: value.creator,
            active: value.active,
            sn: value.sn,
            properties: value.properties,
            new_owner: value.new_owner
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SignaturesInfo {
    pub subject_id: String,
    pub sn: u64,
    pub signatures_eval: Option<HashSet<ProtocolsSignaturesInfo>>,
    pub signatures_appr: Option<HashSet<ProtocolsSignaturesInfo>>,
    pub signatures_vali: HashSet<ProtocolsSignaturesInfo>,
}

impl From<SignaturesInfoBridge> for SignaturesInfo {
    fn from(value: SignaturesInfoBridge) -> Self {
        let signatures_eval = match value.signatures_eval {
            Some(eval) => Some(eval.iter().map(|x| ProtocolsSignaturesInfo::from(x.clone())).collect()),
            None => None,
        };

        let signatures_appr = match value.signatures_appr {
            Some(appr) => Some(appr.iter().map(|x| ProtocolsSignaturesInfo::from(x.clone())).collect()),
            None => None,
        };

        let signatures_vali = value.signatures_vali.iter().map(|x| ProtocolsSignaturesInfo::from(x.clone())).collect();

        Self {
            subject_id: value.subject_id,
            sn: value.sn,
            signatures_eval,
            signatures_appr,
            signatures_vali,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq, Hash)]
pub enum ProtocolsSignaturesInfo {
    Signature( SignatureInfo ),
    TimeOut( TimeOutResponseInfo ),
}

impl From<ProtocolsSignaturesInfoBridge> for ProtocolsSignaturesInfo {
    fn from(value: ProtocolsSignaturesInfoBridge) -> Self {
        match value {
            ProtocolsSignaturesInfoBridge::Signature(signature_info) => Self::Signature(SignatureInfo::from(signature_info)),
            ProtocolsSignaturesInfoBridge::TimeOut(time_out_response_info) => Self::TimeOut(TimeOutResponseInfo::from(time_out_response_info)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq, Hash)]
pub struct TimeOutResponseInfo {
    pub who: String,
    pub re_trys: u32,
    pub timestamp: String,
}

impl From<TimeOutResponseInfoBridge> for TimeOutResponseInfo {
    fn from(value: TimeOutResponseInfoBridge) -> Self {
       Self { who: value.who, re_trys: value.re_trys, timestamp: value.timestamp }
    }
}