import type { MessageEnvelope } from "./message.js";
import type { ProtocolSubmission } from "./delivery.js";
import { CURRENT_PROTOCOL_VERSION } from "./version.js";

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
