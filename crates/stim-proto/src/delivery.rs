use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{EnvelopeId, MessageId, ReplyHandle};

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
pub struct ProtocolSubmission {
    pub acknowledgement: ProtocolAcknowledgement,
    pub reply: Option<ReplyHandle>,
}
