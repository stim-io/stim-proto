import type { EnvelopeId, MessageId } from "./ids.js";
import type { ReplyHandle } from "./reply.js";

export type DeliveryReceiptResult =
  | "accepted"
  | "rejected"
  | "unreachable"
  | "timed_out";

export interface DeliveryReceipt {
  envelope_id: EnvelopeId;
  result: DeliveryReceiptResult;
  detail?: string;
}

export type AcknowledgementResult =
  | "applied"
  | "version_conflict"
  | "invalid_state_transition"
  | "unsupported_content"
  | "unknown_conversation"
  | "unauthorized_mutation";

export interface ProtocolAcknowledgement {
  ack_envelope_id: EnvelopeId;
  ack_message_id: MessageId;
  ack_version: number;
  ack_result: AcknowledgementResult;
  detail?: string;
}

export interface ProtocolSubmission {
  acknowledgement: ProtocolAcknowledgement;
  reply?: ReplyHandle;
}
