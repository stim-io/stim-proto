import type { InsertOperation, MessageContent, PatchOperation } from "./content.js";
import type {
  ConversationId,
  EndpointId,
  EnvelopeId,
  KeyRef,
  MessageId,
  NodeId,
  ProtocolVersion,
  Timestamp
} from "./ids.js";

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
