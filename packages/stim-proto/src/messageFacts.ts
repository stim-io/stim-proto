import type {
  Address,
  ContentId,
  ConversationId,
  EndpointId,
  FactId,
  LedgerId,
  MessageId,
  NodeId,
  ParticipantId,
  ProtocolVersion,
  RelationId,
  RevisionId,
  Timestamp
} from "./ids.js";

export type MessageFactType =
  | "created"
  | "content_revised"
  | "state_changed"
  | "relation_recorded"
  | "delivery_recorded"
  | "runtime_recorded";

export type MessageKindCode =
  | "text"
  | "html"
  | "asset"
  | "audio"
  | "video"
  | "file"
  | "tool_call"
  | "tool_result"
  | "thinking"
  | "compact"
  | "system"
  | "extension";

export interface MessageKind {
  code: MessageKindCode;
  extension?: string;
}

export type MessageProjectionState =
  | "visible"
  | "hidden"
  | "redacted"
  | "deleted"
  | "superseded";

export interface MessageFactSource {
  source_kind: string;
  node_id?: NodeId;
  endpoint_id?: EndpointId;
  agent_id?: string;
  instance_id?: string;
}

export interface MessageContentRef {
  content_id: ContentId;
  revision_id?: RevisionId;
  kind: MessageKind;
  storage: ContentStorageRef;
  mime_type?: string;
  byte_size?: number;
  checksum?: string;
}

export type ContentStorageRef =
  | {
      storage_kind: "inline";
      body: unknown;
    }
  | {
      storage_kind: "table";
      table: string;
      key: string;
    }
  | {
      storage_kind: "object";
      object: ObjectStorageRef;
    }
  | {
      storage_kind: "external";
      uri: string;
    };

export interface ObjectStorageRef {
  bucket?: string;
  key: string;
  uri?: string;
}

export interface TextContentRecord {
  content_id: ContentId;
  revision_id: RevisionId;
  text: string;
  format?: string;
  language?: string;
  metadata?: Record<string, unknown>;
}

export interface HtmlContentRecord {
  content_id: ContentId;
  revision_id: RevisionId;
  html: string;
  sanitized?: boolean;
  metadata?: Record<string, unknown>;
}

export interface BlobContentRecord {
  content_id: ContentId;
  revision_id: RevisionId;
  object: ObjectStorageRef;
  mime_type: string;
  byte_size?: number;
  checksum?: string;
  metadata?: Record<string, unknown>;
}

export interface ToolCallContentRecord {
  content_id: ContentId;
  revision_id: RevisionId;
  tool_call_id: string;
  tool_name: string;
  arguments: unknown;
  metadata?: Record<string, unknown>;
}

export interface ToolResultContentRecord {
  content_id: ContentId;
  revision_id: RevisionId;
  tool_call_id: string;
  result?: unknown;
  error_text?: string;
  metadata?: Record<string, unknown>;
}

export interface ThinkingContentRecord {
  content_id: ContentId;
  revision_id: RevisionId;
  text: string;
  visibility?: string;
  metadata?: Record<string, unknown>;
}

export interface CompactContentRecord {
  content_id: ContentId;
  revision_id: RevisionId;
  summary: string;
  covered: MessageRangeRef[];
  metadata?: Record<string, unknown>;
}

export type MessageRelationType =
  | "reply_to"
  | "quotes"
  | "forwards"
  | "revises"
  | "redacts"
  | "compacts"
  | "tool_result_of"
  | "references"
  | "derived_from"
  | "extension";

export interface MessageRelation {
  relation_id: RelationId;
  relation_type: MessageRelationType;
  extension?: string;
  source_message_id: MessageId;
  target: MessageRelationTarget;
  created_at: Timestamp;
  metadata?: Record<string, unknown>;
}

export type MessageRelationTarget =
  | {
      target_kind: "message";
      message_id: MessageId;
    }
  | {
      target_kind: "fact";
      fact_id: FactId;
    }
  | {
      target_kind: "content";
      content_id: ContentId;
    }
  | {
      target_kind: "range";
      range: MessageRangeRef;
    }
  | {
      target_kind: "external";
      uri: string;
    };

export interface MessageRangeRef {
  ledger_id: LedgerId;
  conversation_id?: ConversationId;
  start_seq?: number;
  end_seq?: number;
  message_ids: MessageId[];
}

export interface MessageFactEnvelope {
  protocol_version: ProtocolVersion;
  fact_id: FactId;
  fact_type: MessageFactType;
  ledger_id: LedgerId;
  conversation_id?: ConversationId;
  message_id: MessageId;
  participant_id: ParticipantId;
  kind: MessageKind;
  occurred_at: Timestamp;
  observed_at?: Timestamp;
  ledger_seq?: number;
  causation_id?: FactId;
  correlation_id?: string;
  source: MessageFactSource;
  content_ref?: MessageContentRef;
  relation?: MessageRelation;
  projection_state?: MessageProjectionState;
  metadata?: Record<string, unknown>;
}

export interface MessageCurrentProjection {
  protocol_version: ProtocolVersion;
  ledger_id: LedgerId;
  conversation_id?: ConversationId;
  message_id: MessageId;
  participant_id: ParticipantId;
  kind: MessageKind;
  projection_state: MessageProjectionState;
  current_revision_id?: RevisionId;
  current_content_ref?: MessageContentRef;
  created_at: Timestamp;
  updated_at: Timestamp;
  deleted_at?: Timestamp;
  last_fact_id: FactId;
}

export interface MessageDeliveryFact {
  fact_id: FactId;
  message_id: MessageId;
  participant_id: ParticipantId;
  endpoint_id?: EndpointId;
  address?: Address;
  delivery_state: string;
  occurred_at: Timestamp;
  metadata?: Record<string, unknown>;
}
