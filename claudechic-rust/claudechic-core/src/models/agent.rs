use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;
use crate::models::ChatItem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentStatus {
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "needs_input")]
    NeedsInput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionMode {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "accept_edits")]
    AcceptEdits,
    #[serde(rename = "plan")]
    Plan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub cwd: PathBuf,
    pub worktree: Option<String>,
    pub status: AgentStatus,
    pub messages: Vec<ChatItem>,
    pub permission_mode: PermissionMode,
    pub session_id: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Agent {
    pub fn new(name: String, cwd: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            cwd,
            worktree: None,
            status: AgentStatus::Idle,
            messages: Vec::new(),
            permission_mode: PermissionMode::Default,
            session_id: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    pub fn set_status(&mut self, status: AgentStatus) {
        self.status = status;
        self.updated_at = chrono::Utc::now();
    }

    pub fn set_permission_mode(&mut self, mode: PermissionMode) {
        self.permission_mode = mode;
        self.updated_at = chrono::Utc::now();
    }

    pub fn add_message(&mut self, message: ChatItem) {
        self.messages.push(message);
        self.updated_at = chrono::Utc::now();
    }

    pub fn cycle_permission_mode(&mut self) {
        self.permission_mode = match self.permission_mode {
            PermissionMode::Default => PermissionMode::AcceptEdits,
            PermissionMode::AcceptEdits => PermissionMode::Plan,
            PermissionMode::Plan => PermissionMode::Default,
        };
        self.updated_at = chrono::Utc::now();
    }
}
