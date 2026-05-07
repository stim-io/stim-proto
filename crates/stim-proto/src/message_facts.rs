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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CURRENT_PROTOCOL_VERSION;
    use serde_json::json;

    #[test]
    fn message_fact_roundtrip_preserves_fact_and_content_refs() {
        let fact = MessageFactEnvelope {
            protocol_version: CURRENT_PROTOCOL_VERSION.into(),
            fact_id: "fact-1".into(),
            fact_type: MessageFactType::Created,
            ledger_id: "stim-server:ledger:main".into(),
            conversation_id: Some("conv-1".into()),
            message_id: "msg-1".into(),
            participant_id: "participant-santi".into(),
            kind: MessageKind::core(MessageKindCode::Text),
            occurred_at: "2026-05-07T00:00:00Z".into(),
            observed_at: Some("2026-05-07T00:00:01Z".into()),
            ledger_seq: Some(42),
            causation_id: None,
            correlation_id: Some("corr-1".into()),
            source: MessageFactSource {
                source_kind: "stim-server".into(),
                node_id: None,
                endpoint_id: Some("endpoint-santi".into()),
                agent_id: Some("agent-santi".into()),
                instance_id: Some("local-santi".into()),
            },
            content_ref: Some(MessageContentRef {
                content_id: "content-1".into(),
                revision_id: Some("rev-1".into()),
                kind: MessageKind::core(MessageKindCode::Text),
                storage: ContentStorageRef::Table {
                    table: "message_text".into(),
                    key: "content-1".into(),
                },
                mime_type: Some("text/plain".into()),
                byte_size: Some(11),
                checksum: Some("sha256:abc".into()),
            }),
            relation: None,
            projection_state: Some(MessageProjectionState::Visible),
            metadata: Some(json!({"projection": "columnar-ready"})),
        };

        let encoded = serde_json::to_value(&fact).unwrap();
        assert_eq!(encoded["fact_type"], "created");
        assert_eq!(encoded["kind"]["code"], "text");
        assert_eq!(encoded["content_ref"]["storage"]["storage_kind"], "table");

        let decoded: MessageFactEnvelope = serde_json::from_value(encoded).unwrap();
        assert_eq!(decoded.ledger_seq, Some(42));
        assert_eq!(decoded.participant_id, "participant-santi");
    }

    #[test]
    fn compact_relation_can_cover_message_ranges_without_content_inline() {
        let relation = MessageRelation {
            relation_id: "rel-1".into(),
            relation_type: MessageRelationType::Compacts,
            extension: None,
            source_message_id: "msg-compact-1".into(),
            target: MessageRelationTarget::Range {
                range: MessageRangeRef {
                    ledger_id: "santi:runtime:soul-1".into(),
                    conversation_id: Some("session-1".into()),
                    start_seq: Some(1),
                    end_seq: Some(10),
                    message_ids: vec![],
                },
            },
            created_at: "2026-05-07T00:00:00Z".into(),
            metadata: None,
        };

        let encoded = serde_json::to_value(&relation).unwrap();
        assert_eq!(encoded["relation_type"], "compacts");
        assert_eq!(encoded["target"]["target_kind"], "range");
    }
}
