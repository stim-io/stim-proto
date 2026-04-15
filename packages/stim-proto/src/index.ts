export const STIM_PROTO_VERSION = "0.1.0-dev.0" as const;
export const CURRENT_PROTOCOL_VERSION = "stim/0.1" as const;

export type NodeId = string;
export type EndpointId = string;
export type ConversationId = string;
export type EnvelopeId = string;
export type MessageId = string;
export type ReplyId = string;
export type ProtocolVersion = string;
export type Address = string;
export type Timestamp = string;
export type KeyRef = string;

export interface EndpointDeclaration {
  endpoint_id: EndpointId;
  node_id: NodeId;
  display_label?: string;
  endpoint_kind?: string;
  supported_protocol_versions: ProtocolVersion[];
  supported_carriers: string[];
  content_capabilities: string[];
  security_capabilities: string[];
  declared_features: string[];
}

export interface DiscoveryRecord {
  node_id: NodeId;
  endpoint_declaration: EndpointDeclaration;
  carrier_kind: string;
  addresses: Address[];
  protocol_versions: ProtocolVersion[];
}

export interface DeliveryTarget {
  node_id: NodeId;
  carrier_kind: string;
  selected_address: Address;
  protocol_version: ProtocolVersion;
}

export interface SessionBootstrap {
  participants: EndpointId[];
  created_by: EndpointId;
  created_at: Timestamp;
}

export interface SenderAssertion {
  assertion_kind: string;
  reference?: string;
}

export interface EncryptionScope {
  scope_kind: string;
  session_id?: ConversationId;
}

export type MessageState = "pending" | "fixed";
export type MessageOperation = "create" | "patch" | "insert" | "remove" | "fix";

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

export type ReplyStatus = "pending" | "streaming" | "completed" | "failed";

export interface ReplyHandle {
  reply_id: ReplyId;
  conversation_id: ConversationId;
  message_id: MessageId;
  status: ReplyStatus;
}

export interface ReplyFailure {
  code: string;
  message: string;
}

export interface ReplySnapshot {
  reply_id: ReplyId;
  conversation_id: ConversationId;
  message_id: MessageId;
  status: ReplyStatus;
  output_text: string;
  error?: ReplyFailure;
}

export type ReplyEventKind =
  | {
      type: "output_text_delta";
      delta: string;
    }
  | {
      type: "completed";
    }
  | {
      type: "failed";
      error: ReplyFailure;
    };

export interface ReplyEvent {
  reply_id: ReplyId;
  sequence: number;
  event: ReplyEventKind;
}

export interface ProtocolSubmission {
  acknowledgement: ProtocolAcknowledgement;
  reply?: ReplyHandle;
}

export interface LayoutHint {
  layout_family?: string;
  min_height_px?: number;
  max_height_px?: number;
  vertical_pressure?: string;
  metadata?: Record<string, unknown>;
}

export interface MessageContent {
  parts: ContentPart[];
  layout_hint?: LayoutHint;
}

interface BaseContentPart {
  part_id: string;
  revision: number;
  metadata?: Record<string, unknown>;
}

export interface TextPart extends BaseContentPart {
  type: "text";
  text: string;
}

export interface AssetRefPart extends BaseContentPart {
  type: "asset_ref";
  asset_ref: string;
  mime_type?: string;
}

export interface StimDomFragmentPayload {
  format: "stim-dom-fragment/v1";
  tree: Record<string, unknown>;
  bindings?: Record<string, unknown>;
}

export interface RawHtmlDomFragmentPayload {
  format: "raw_html";
  html: string;
  bindings?: Record<string, unknown>;
}

export interface DomFragmentPart extends BaseContentPart {
  type: "dom_fragment";
  payload: StimDomFragmentPayload | RawHtmlDomFragmentPayload;
}

export interface CapabilityRefPart extends BaseContentPart {
  type: "capability_ref";
  capability_kind: string;
  capability_id: string;
  input_schema_ref?: string;
  resource_ref?: string;
}

export type ContentPart =
  | TextPart
  | AssetRefPart
  | DomFragmentPart
  | CapabilityRefPart;

export interface PatchOperation {
  index: number;
  merge: Record<string, unknown>;
}

export interface InsertOperation {
  index: number;
  part: ContentPart;
}

export type MutationPayload =
  | {
      operation: "create";
      content: MessageContent;
    }
  | {
      operation: "patch";
      patches: PatchOperation[];
    }
  | {
      operation: "insert";
      items: InsertOperation[];
    }
  | {
      operation: "remove";
      indexes: number[];
    }
  | {
      operation: "fix";
    };

export interface MessageEnvelope {
  protocol_version: ProtocolVersion;
  envelope_id: EnvelopeId;
  message_id: MessageId;
  conversation_id: ConversationId;
  sender_node_id: NodeId;
  sender_endpoint_id: EndpointId;
  created_at: Timestamp;
  session_bootstrap?: SessionBootstrap;
  sender_assertion?: SenderAssertion;
  encryption_scope?: EncryptionScope;
  recipient_key_refs: KeyRef[];
  signature_ref?: string;
  integrity_ref?: string;
  state: MessageState;
  operation: MessageOperation;
  base_version?: number;
  new_version: number;
  payload: MutationPayload;
}

export const exampleEnvelope: MessageEnvelope = {
  protocol_version: CURRENT_PROTOCOL_VERSION,
  envelope_id: "env-1",
  message_id: "msg-1",
  conversation_id: "conv-1",
  sender_node_id: "node-a",
  sender_endpoint_id: "endpoint-a",
  created_at: "2026-04-14T00:00:00Z",
  session_bootstrap: {
    participants: ["endpoint-a", "endpoint-b"],
    created_by: "endpoint-a",
    created_at: "2026-04-14T00:00:00Z"
  },
  sender_assertion: {
    assertion_kind: "self-signed"
  },
  encryption_scope: {
    scope_kind: "recipient_set",
    session_id: "conv-1"
  },
  recipient_key_refs: ["key-a"],
  signature_ref: "sig-1",
  integrity_ref: "sha256:abc",
  state: "pending",
  operation: "create",
  new_version: 1,
  payload: {
    operation: "create",
    content: {
      parts: [
        {
          type: "dom_fragment",
          part_id: "part-1",
          revision: 1,
          payload: {
            format: "stim-dom-fragment/v1",
            tree: { tag: "div", children: [] },
            bindings: { action: "reply" }
          }
        }
      ],
      layout_hint: {
        layout_family: "card",
        min_height_px: 120,
        vertical_pressure: "expand"
      }
    }
  }
};

export const exampleProtocolSubmission: ProtocolSubmission = {
  acknowledgement: {
    ack_envelope_id: "ack-env-1",
    ack_message_id: "msg-1",
    ack_version: 3,
    ack_result: "applied",
    detail: "applied"
  },
  reply: {
    reply_id: "reply-1",
    conversation_id: "conv-1",
    message_id: "msg-1",
    status: "pending"
  }
};
