use serde_json::json;
use stim_proto::*;

mod envelope {
    use super::*;

    #[test]
    fn preserves_operation_shape() {
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

mod submission {
    use super::*;

    #[test]
    fn preserves_reply_handle() {
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
}

mod reply_event {
    use super::*;

    #[test]
    fn preserves_failed_variant() {
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
