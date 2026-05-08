use serde_json::json;
use stim_proto::*;

mod fact {
    use super::*;

    #[test]
    fn preserves_refs() {
        let fact = MessageFactEnvelope {
            protocol_version: CURRENT_PROTOCOL_VERSION.into(),
            fact_id: "fact-1".into(),
            fact_type: MessageFactType::Created,
            ledger_id: "stim-server:ledger:main".into(),
            conversation_id: Some("conv-1".into()),
            message_id: "msg-1".into(),
            participant_id: "participant-santi".into(),
            kind: MessageKind::core(MessageKindCode::Text),
            occurred_at: "2026-05-07T00:00:00Z".into(),
            observed_at: Some("2026-05-07T00:00:01Z".into()),
            ledger_seq: Some(42),
            causation_id: None,
            correlation_id: Some("corr-1".into()),
            source: MessageFactSource {
                source_kind: "stim-server".into(),
                node_id: None,
                endpoint_id: Some("endpoint-santi".into()),
                agent_id: Some("agent-santi".into()),
                instance_id: Some("local-santi".into()),
            },
            content_ref: Some(MessageContentRef {
                content_id: "content-1".into(),
                revision_id: Some("rev-1".into()),
                kind: MessageKind::core(MessageKindCode::Text),
                storage: ContentStorageRef::Table {
                    table: "message_text".into(),
                    key: "content-1".into(),
                },
                mime_type: Some("text/plain".into()),
                byte_size: Some(11),
                checksum: Some("sha256:abc".into()),
            }),
            relation: None,
            projection_state: Some(MessageProjectionState::Visible),
            metadata: Some(json!({"projection": "columnar-ready"})),
        };

        let encoded = serde_json::to_value(&fact).unwrap();
        assert_eq!(encoded["fact_type"], "created");
        assert_eq!(encoded["kind"]["code"], "text");
        assert_eq!(encoded["content_ref"]["storage"]["storage_kind"], "table");

        let decoded: MessageFactEnvelope = serde_json::from_value(encoded).unwrap();
        assert_eq!(decoded.ledger_seq, Some(42));
        assert_eq!(decoded.participant_id, "participant-santi");
    }
}

mod compact_relation {
    use super::*;

    #[test]
    fn covers_ranges() {
        let relation = MessageRelation {
            relation_id: "rel-1".into(),
            relation_type: MessageRelationType::Compacts,
            extension: None,
            source_message_id: "msg-compact-1".into(),
            target: MessageRelationTarget::Range {
                range: MessageRangeRef {
                    ledger_id: "santi:runtime:soul-1".into(),
                    conversation_id: Some("session-1".into()),
                    start_seq: Some(1),
                    end_seq: Some(10),
                    message_ids: vec![],
                },
            },
            created_at: "2026-05-07T00:00:00Z".into(),
            metadata: None,
        };

        let encoded = serde_json::to_value(&relation).unwrap();
        assert_eq!(encoded["relation_type"], "compacts");
        assert_eq!(encoded["target"]["target_kind"], "range");
    }
}
