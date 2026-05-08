use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use crate::{
    Address, ContentId, ConversationId, EndpointId, FactId, LedgerId, MessageId, NodeId,
    ParticipantId, ProtocolVersion, RelationId, RevisionId, Timestamp,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MessageFactType {
    Created,
    ContentRevised,
    StateChanged,
    RelationRecorded,
    DeliveryRecorded,
    RuntimeRecorded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MessageKindCode {
    Text,
    Html,
    Asset,
    Audio,
    Video,
    File,
    ToolCall,
    ToolResult,
    Thinking,
    Compact,
    System,
    Extension,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct MessageKind {
    pub code: MessageKindCode,
    pub extension: Option<String>,
}

impl MessageKind {
    pub fn core(code: MessageKindCode) -> Self {
        Self {
            code,
            extension: None,
        }
    }

    pub fn extension(extension: impl Into<String>) -> Self {
        Self {
            code: MessageKindCode::Extension,
            extension: Some(extension.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MessageProjectionState {
    Visible,
    Hidden,
    Redacted,
    Deleted,
    Superseded,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct MessageFactSource {
    pub source_kind: String,
    pub node_id: Option<NodeId>,
    pub endpoint_id: Option<EndpointId>,
    pub agent_id: Option<String>,
    pub instance_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct MessageContentRef {
    pub content_id: ContentId,
    pub revision_id: Option<RevisionId>,
    pub kind: MessageKind,
    pub storage: ContentStorageRef,
    pub mime_type: Option<String>,
    pub byte_size: Option<u64>,
    pub checksum: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "storage_kind", rename_all = "snake_case")]
pub enum ContentStorageRef {
    Inline { body: Value },
    Table { table: String, key: String },
    Object { object: ObjectStorageRef },
    External { uri: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ObjectStorageRef {
    pub bucket: Option<String>,
    pub key: String,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TextContentRecord {
    pub content_id: ContentId,
    pub revision_id: RevisionId,
    pub text: String,
    pub format: Option<String>,
    pub language: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct HtmlContentRecord {
    pub content_id: ContentId,
    pub revision_id: RevisionId,
    pub html: String,
    pub sanitized: Option<bool>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct BlobContentRecord {
    pub content_id: ContentId,
    pub revision_id: RevisionId,
    pub object: ObjectStorageRef,
    pub mime_type: String,
    pub byte_size: Option<u64>,
    pub checksum: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ToolCallContentRecord {
    pub content_id: ContentId,
    pub revision_id: RevisionId,
    pub tool_call_id: String,
    pub tool_name: String,
    pub arguments: Value,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ToolResultContentRecord {
    pub content_id: ContentId,
    pub revision_id: RevisionId,
    pub tool_call_id: String,
    pub result: Option<Value>,
    pub error_text: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ThinkingContentRecord {
    pub content_id: ContentId,
    pub revision_id: RevisionId,
    pub text: String,
    pub visibility: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CompactContentRecord {
    pub content_id: ContentId,
    pub revision_id: RevisionId,
    pub summary: String,
    pub covered: Vec<MessageRangeRef>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MessageRelationType {
    ReplyTo,
    Quotes,
    Forwards,
    Revises,
    Redacts,
    Compacts,
    ToolResultOf,
    References,
    DerivedFrom,
    Extension,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct MessageRelation {
    pub relation_id: RelationId,
    pub relation_type: MessageRelationType,
    pub extension: Option<String>,
    pub source_message_id: MessageId,
    pub target: MessageRelationTarget,
    pub created_at: Timestamp,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "target_kind", rename_all = "snake_case")]
pub enum MessageRelationTarget {
    Message { message_id: MessageId },
    Fact { fact_id: FactId },
    Content { content_id: ContentId },
    Range { range: MessageRangeRef },
    External { uri: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct MessageRangeRef {
    pub ledger_id: LedgerId,
    pub conversation_id: Option<ConversationId>,
    pub start_seq: Option<u64>,
    pub end_seq: Option<u64>,
    pub message_ids: Vec<MessageId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct MessageFactEnvelope {
    pub protocol_version: ProtocolVersion,
    pub fact_id: FactId,
    pub fact_type: MessageFactType,
    pub ledger_id: LedgerId,
    pub conversation_id: Option<ConversationId>,
    pub message_id: MessageId,
    pub participant_id: ParticipantId,
    pub kind: MessageKind,
    pub occurred_at: Timestamp,
    pub observed_at: Option<Timestamp>,
    pub ledger_seq: Option<u64>,
    pub causation_id: Option<FactId>,
    pub correlation_id: Option<String>,
    pub source: MessageFactSource,
    pub content_ref: Option<MessageContentRef>,
    pub relation: Option<MessageRelation>,
    pub projection_state: Option<MessageProjectionState>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct MessageCurrentProjection {
    pub protocol_version: ProtocolVersion,
    pub ledger_id: LedgerId,
    pub conversation_id: Option<ConversationId>,
    pub message_id: MessageId,
    pub participant_id: ParticipantId,
    pub kind: MessageKind,
    pub projection_state: MessageProjectionState,
    pub current_revision_id: Option<RevisionId>,
    pub current_content_ref: Option<MessageContentRef>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub deleted_at: Option<Timestamp>,
    pub last_fact_id: FactId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct MessageDeliveryFact {
    pub fact_id: FactId,
    pub message_id: MessageId,
    pub participant_id: ParticipantId,
    pub endpoint_id: Option<EndpointId>,
    pub address: Option<Address>,
    pub delivery_state: String,
    pub occurred_at: Timestamp,
    pub metadata: Option<Value>,
}
