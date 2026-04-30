use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    ConversationId, EndpointId, EnvelopeId, InsertOperation, KeyRef, MessageContent, MessageId,
    NodeId, PatchOperation, ProtocolVersion, Timestamp,
};

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
