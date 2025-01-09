use kore_bridge::{RequestData as RequestDataBridge, GovsData as GovsDataBridge, RegisterData as RegisterDataBridge, RequestDB as RequestDBridge};
use serde::{Deserialize, Serialize};
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
pub struct RequestDB {
    status: String,
    version: u64,
    error: Option<String>,
}

impl From<RequestDBridge> for RequestDB {
    fn from(value: RequestDBridge) -> Self {
        Self { status: value.status, version: value.version, error: value.error }
    }
}