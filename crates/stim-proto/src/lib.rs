//! Shared protocol contracts for `stim.io` participants.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

/// Current crate version exposed for downstream bootstrap checks.
pub const STIM_PROTO_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Current shared protocol version string for the first execution wave.
pub const CURRENT_PROTOCOL_VERSION: &str = "stim/0.1";

pub type NodeId = String;
pub type EndpointId = String;
pub type ConversationId = String;
pub type EnvelopeId = String;
pub type MessageId = String;
pub type ReplyId = String;
pub type ProtocolVersion = String;
pub type Address = String;
pub type Timestamp = String;
pub type KeyRef = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct EndpointDeclaration {
    pub endpoint_id: EndpointId,
    pub node_id: NodeId,
    pub display_label: Option<String>,
    pub endpoint_kind: Option<String>,
    pub supported_protocol_versions: Vec<ProtocolVersion>,
    pub supported_carriers: Vec<String>,
    pub content_capabilities: Vec<String>,
    pub security_capabilities: Vec<String>,
    pub declared_features: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct DiscoveryRecord {
    pub node_id: NodeId,
    pub endpoint_declaration: EndpointDeclaration,
    pub carrier_kind: String,
    pub addresses: Vec<Address>,
    pub protocol_versions: Vec<ProtocolVersion>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct DeliveryTarget {
    pub node_id: NodeId,
    pub carrier_kind: String,
    pub selected_address: Address,
    pub protocol_version: ProtocolVersion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct SessionBootstrap {
    pub participants: Vec<EndpointId>,
    pub created_by: EndpointId,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct SenderAssertion {
    pub assertion_kind: String,
    pub reference: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct EncryptionScope {
    pub scope_kind: String,
    pub session_id: Option<ConversationId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MessageState {
    Pending,
    Fixed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MessageOperation {
    Create,
    Patch,
    Insert,
    Remove,
    Fix,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryReceiptResult {
    Accepted,
    Rejected,
    Unreachable,
    TimedOut,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct DeliveryReceipt {
    pub envelope_id: EnvelopeId,
    pub result: DeliveryReceiptResult,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcknowledgementResult {
    Applied,
    VersionConflict,
    InvalidStateTransition,
    UnsupportedContent,
    UnknownConversation,
    UnauthorizedMutation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ProtocolAcknowledgement {
    pub ack_envelope_id: EnvelopeId,
    pub ack_message_id: MessageId,
    pub ack_version: u64,
    pub ack_result: AcknowledgementResult,
    pub detail: Option<String>,
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ProtocolSubmission {
    pub acknowledgement: ProtocolAcknowledgement,
    pub reply: Option<ReplyHandle>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct LayoutHint {
    pub layout_family: Option<String>,
    pub min_height_px: Option<u32>,
    pub max_height_px: Option<u32>,
    pub vertical_pressure: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct MessageContent {
    pub parts: Vec<ContentPart>,
    pub layout_hint: Option<LayoutHint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text(TextPart),
    AssetRef(AssetRefPart),
    DomFragment(DomFragmentPart),
    CapabilityRef(CapabilityRefPart),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TextPart {
    pub part_id: String,
    pub revision: u64,
    pub metadata: Option<Value>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct AssetRefPart {
    pub part_id: String,
    pub revision: u64,
    pub metadata: Option<Value>,
    pub asset_ref: String,
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "format", rename_all = "snake_case")]
pub enum DomFragmentPayload {
    #[serde(rename = "stim-dom-fragment/v1")]
    StimDomFragmentV1 {
        tree: Value,
        bindings: Option<Value>,
    },
    RawHtml {
        html: String,
        bindings: Option<Value>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct DomFragmentPart {
    pub part_id: String,
    pub revision: u64,
    pub metadata: Option<Value>,
    #[serde(flatten)]
    pub payload: DomFragmentPayload,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CapabilityRefPart {
    pub part_id: String,
    pub revision: u64,
    pub metadata: Option<Value>,
    pub capability_kind: String,
    pub capability_id: String,
    pub input_schema_ref: Option<String>,
    pub resource_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PatchOperation {
    pub index: usize,
    pub merge: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct InsertOperation {
    pub index: usize,
    pub part: ContentPart,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum MutationPayload {
    Create { content: MessageContent },
    Patch { patches: Vec<PatchOperation> },
    Insert { items: Vec<InsertOperation> },
    Remove { indexes: Vec<usize> },
    Fix {},
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct MessageEnvelope {
    pub protocol_version: ProtocolVersion,
    pub envelope_id: EnvelopeId,
    pub message_id: MessageId,
    pub conversation_id: ConversationId,
    pub sender_node_id: NodeId,
    pub sender_endpoint_id: EndpointId,
    pub created_at: Timestamp,
    pub session_bootstrap: Option<SessionBootstrap>,
    pub sender_assertion: Option<SenderAssertion>,
    pub encryption_scope: Option<EncryptionScope>,
    pub recipient_key_refs: Vec<KeyRef>,
    pub signature_ref: Option<String>,
    pub integrity_ref: Option<String>,
    pub state: MessageState,
    pub operation: MessageOperation,
    pub base_version: Option<u64>,
    pub new_version: u64,
    pub payload: MutationPayload,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn envelope_roundtrip_preserves_operation_shape() {
        let envelope = MessageEnvelope {
            protocol_version: CURRENT_PROTOCOL_VERSION.to_string(),
            envelope_id: "env-1".into(),
            message_id: "msg-1".into(),
            conversation_id: "conv-1".into(),
            sender_node_id: "node-a".into(),
            sender_endpoint_id: "endpoint-a".into(),
            created_at: "2026-04-14T00:00:00Z".into(),
            session_bootstrap: Some(SessionBootstrap {
                participants: vec!["endpoint-a".into(), "endpoint-b".into()],
                created_by: "endpoint-a".into(),
                created_at: "2026-04-14T00:00:00Z".into(),
            }),
            sender_assertion: Some(SenderAssertion {
                assertion_kind: "self-signed".into(),
                reference: None,
            }),
            encryption_scope: Some(EncryptionScope {
                scope_kind: "recipient_set".into(),
                session_id: Some("conv-1".into()),
            }),
            recipient_key_refs: vec!["key-a".into()],
            signature_ref: Some("sig-1".into()),
            integrity_ref: Some("sha256:abc".into()),
            state: MessageState::Pending,
            operation: MessageOperation::Create,
            base_version: None,
            new_version: 1,
            payload: MutationPayload::Create {
                content: MessageContent {
                    parts: vec![ContentPart::DomFragment(DomFragmentPart {
                        part_id: "part-1".into(),
                        revision: 1,
                        metadata: None,
                        payload: DomFragmentPayload::StimDomFragmentV1 {
                            tree: json!({"tag": "div", "children": []}),
                            bindings: Some(json!({"action": "reply"})),
                        },
                    })],
                    layout_hint: Some(LayoutHint {
                        layout_family: Some("card".into()),
                        min_height_px: Some(120),
                        max_height_px: None,
                        vertical_pressure: Some("expand".into()),
                        metadata: None,
                    }),
                },
            },
        };

        let encoded = serde_json::to_value(&envelope).unwrap();
        assert_eq!(encoded["operation"], "create");
        assert_eq!(encoded["payload"]["operation"], "create");

        let decoded: MessageEnvelope = serde_json::from_value(encoded).unwrap();
        assert_eq!(decoded.operation, MessageOperation::Create);
        assert_eq!(decoded.new_version, 1);
    }

    #[test]
    fn protocol_submission_roundtrip_preserves_reply_handle_shape() {
        let submission = ProtocolSubmission {
            acknowledgement: ProtocolAcknowledgement {
                ack_envelope_id: "ack-env-1".into(),
                ack_message_id: "msg-1".into(),
                ack_version: 3,
                ack_result: AcknowledgementResult::Applied,
                detail: Some("applied".into()),
            },
            reply: Some(ReplyHandle {
                reply_id: "reply-1".into(),
                conversation_id: "conv-1".into(),
                message_id: "msg-1".into(),
                status: ReplyStatus::Pending,
            }),
        };

        let encoded = serde_json::to_value(&submission).unwrap();
        assert_eq!(encoded["acknowledgement"]["ack_result"], "applied");
        assert_eq!(encoded["reply"]["status"], "pending");

        let decoded: ProtocolSubmission = serde_json::from_value(encoded).unwrap();
        assert_eq!(decoded.reply.unwrap().reply_id, "reply-1");
    }

    #[test]
    fn reply_event_roundtrip_preserves_failed_variant() {
        let event = ReplyEvent {
            reply_id: "reply-1".into(),
            sequence: 2,
            event: ReplyEventKind::Failed {
                error: ReplyFailure {
                    code: "provider_error".into(),
                    message: "upstream failed".into(),
                },
            },
        };

        let encoded = serde_json::to_value(&event).unwrap();
        assert_eq!(encoded["event"]["type"], "failed");
        assert_eq!(encoded["event"]["error"]["code"], "provider_error");

        let decoded: ReplyEvent = serde_json::from_value(encoded).unwrap();
        assert_eq!(decoded.sequence, 2);
    }
}
