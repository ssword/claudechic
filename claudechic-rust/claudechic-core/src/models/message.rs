use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAttachment {
    pub path: String,
    pub filename: String,
    pub media_type: String,
    pub base64_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContent {
    pub text: String,
    pub images: Vec<ImageAttachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBlock {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUseBlock {
    pub id: String,
    pub name: String,
    pub input: serde_json::Value,
    pub parent_tool_use_id: Option<String>,
    pub result: Option<String>,
    pub is_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum AssistantBlock {
    #[serde(rename = "text")]
    Text(TextBlock),
    #[serde(rename = "tool_use")]
    ToolUse(Box<ToolUseBlock>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantContent {
    pub blocks: Vec<AssistantBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "role", content = "content")]
pub enum MessageContent {
    #[serde(rename = "user")]
    User(UserContent),
    #[serde(rename = "assistant")]
    Assistant(AssistantContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatItem {
    pub id: Uuid,
    pub content: MessageContent,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ChatItem {
    pub fn new(content: MessageContent) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn user(text: String) -> Self {
        Self::new(MessageContent::User(UserContent {
            text,
            images: Vec::new(),
        }))
    }

    pub fn assistant_text(text: String) -> Self {
        Self::new(MessageContent::Assistant(AssistantContent {
            blocks: vec![AssistantBlock::Text(TextBlock { text })],
        }))
    }

    pub fn is_user(&self) -> bool {
        matches!(self.content, MessageContent::User(_))
    }

    pub fn is_assistant(&self) -> bool {
        matches!(self.content, MessageContent::Assistant(_))
    }
}
