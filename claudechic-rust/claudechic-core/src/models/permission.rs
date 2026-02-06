use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionResult {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "allow_session")]
    AllowSession,
    #[serde(rename = "allow_all")]
    AllowAll,
}

pub struct PermissionRequest {
    pub tool_name: String,
    pub tool_input: serde_json::Value,
    pub tx: oneshot::Sender<PermissionResult>,
}

impl std::fmt::Debug for PermissionRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PermissionRequest")
            .field("tool_name", &self.tool_name)
            .field("tool_input", &self.tool_input)
            .finish()
    }
}
