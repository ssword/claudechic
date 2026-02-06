use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUse {
    pub id: String,
    pub name: String,
    pub input: serde_json::Value,
    pub parent_tool_use_id: Option<String>,
    pub result: Option<String>,
    pub is_error: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ToolUse {
    pub fn new(name: String, input: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            input,
            parent_tool_use_id: None,
            result: None,
            is_error: false,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn with_parent(mut self, parent_id: String) -> Self {
        self.parent_tool_use_id = Some(parent_id);
        self
    }

    pub fn set_result(&mut self, result: String, is_error: bool) {
        self.result = Some(result);
        self.is_error = is_error;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_use_id: String,
    pub content: String,
    pub is_error: bool,
}

impl ToolResult {
    pub fn new(tool_use_id: String, content: String, is_error: bool) -> Self {
        Self {
            tool_use_id,
            content,
            is_error,
        }
    }
}
