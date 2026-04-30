import type { ConversationId, MessageId, ReplyId } from "./ids.js";

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
