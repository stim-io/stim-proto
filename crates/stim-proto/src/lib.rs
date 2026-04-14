//! Shared protocol contracts for `stim.io` participants.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Current crate version exposed for downstream bootstrap checks.
pub const STIM_PROTO_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Current shared protocol version string for the first execution wave.
pub const CURRENT_PROTOCOL_VERSION: &str = "stim/0.1";

pub type NodeId = String;
pub type EndpointId = String;
pub type ConversationId = String;
pub type EnvelopeId = String;
pub type MessageId = String;
pub type ProtocolVersion = String;
pub type Address = String;
pub type Timestamp = String;
pub type KeyRef = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveryRecord {
    pub node_id: NodeId,
    pub endpoint_declaration: EndpointDeclaration,
    pub carrier_kind: String,
    pub addresses: Vec<Address>,
    pub protocol_versions: Vec<ProtocolVersion>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeliveryTarget {
    pub node_id: NodeId,
    pub carrier_kind: String,
    pub selected_address: Address,
    pub protocol_version: ProtocolVersion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionBootstrap {
    pub participants: Vec<EndpointId>,
    pub created_by: EndpointId,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SenderAssertion {
    pub assertion_kind: String,
    pub reference: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptionScope {
    pub scope_kind: String,
    pub session_id: Option<ConversationId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageState {
    Pending,
    Fixed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageOperation {
    Create,
    Patch,
    Insert,
    Remove,
    Fix,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryReceiptResult {
    Accepted,
    Rejected,
    Unreachable,
    TimedOut,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeliveryReceipt {
    pub envelope_id: EnvelopeId,
    pub result: DeliveryReceiptResult,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcknowledgementResult {
    Applied,
    VersionConflict,
    InvalidStateTransition,
    UnsupportedContent,
    UnknownConversation,
    UnauthorizedMutation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolAcknowledgement {
    pub ack_envelope_id: EnvelopeId,
    pub ack_message_id: MessageId,
    pub ack_version: u64,
    pub ack_result: AcknowledgementResult,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutHint {
    pub layout_family: Option<String>,
    pub min_height_px: Option<u32>,
    pub max_height_px: Option<u32>,
    pub vertical_pressure: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageContent {
    pub parts: Vec<ContentPart>,
    pub layout_hint: Option<LayoutHint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text(TextPart),
    AssetRef(AssetRefPart),
    DomFragment(DomFragmentPart),
    CapabilityRef(CapabilityRefPart),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextPart {
    pub part_id: String,
    pub revision: u64,
    pub metadata: Option<Value>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetRefPart {
    pub part_id: String,
    pub revision: u64,
    pub metadata: Option<Value>,
    pub asset_ref: String,
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomFragmentPart {
    pub part_id: String,
    pub revision: u64,
    pub metadata: Option<Value>,
    #[serde(flatten)]
    pub payload: DomFragmentPayload,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityRefPart {
    pub part_id: String,
    pub revision: u64,
    pub metadata: Option<Value>,
    pub capability_kind: String,
    pub capability_id: String,
    pub input_schema_ref: Option<String>,
    pub resource_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchOperation {
    pub index: usize,
    pub merge: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsertOperation {
    pub index: usize,
    pub part: ContentPart,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum MutationPayload {
    Create { content: MessageContent },
    Patch { patches: Vec<PatchOperation> },
    Insert { items: Vec<InsertOperation> },
    Remove { indexes: Vec<usize> },
    Fix {},
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
}
