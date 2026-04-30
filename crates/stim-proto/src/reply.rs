use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{ConversationId, MessageId, ReplyId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReplyStatus {
    Pending,
    Streaming,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ReplyHandle {
    pub reply_id: ReplyId,
    pub conversation_id: ConversationId,
    pub message_id: MessageId,
    pub status: ReplyStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ReplyFailure {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ReplySnapshot {
    pub reply_id: ReplyId,
    pub conversation_id: ConversationId,
    pub message_id: MessageId,
    pub status: ReplyStatus,
    pub output_text: String,
    pub error: Option<ReplyFailure>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReplyEventKind {
    OutputTextDelta { delta: String },
    Completed,
    Failed { error: ReplyFailure },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ReplyEvent {
    pub reply_id: ReplyId,
    pub sequence: u64,
    pub event: ReplyEventKind,
}
