use crate::models::{ToolUse, PermissionRequest, ToolResult};
use uuid::Uuid;

#[derive(Debug)]
pub enum AgentEvent {
    TextChunk {
        agent_id: Uuid,
        text: String,
    },
    ToolUse {
        agent_id: Uuid,
        tool: ToolUse,
    },
    ToolResult {
        agent_id: Uuid,
        result: ToolResult,
    },
    Complete {
        agent_id: Uuid,
    },
    Error {
        agent_id: Uuid,
        message: String,
    },
    StatusChanged {
        agent_id: Uuid,
        message: String,
    },
    PermissionNeeded {
        agent_id: Uuid,
        request: PermissionRequest,
    },
}
