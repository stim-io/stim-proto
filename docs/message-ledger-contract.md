# Message Ledger Contract

This document defines the shared message-ledger semantics owned by `stim-proto`.

It is a protocol contract, not a storage implementation. `stim-server`, `santi`,
and local `stim` controller events may map these contracts into different
stores, tables, queues, or projections while preserving the same ids and
relations.

## Ownership

- `stim-server` owns durable product IM ledger truth.
- `santi` owns agent/runtime ledger truth, including runtime artifacts such as
  tool calls, tool results, thinking, and compact records.
- `stim` controller owns local operation events and forwarding observations; it
  does not replace the product ledger.
- `stim-proto` owns the shared envelope, content-reference, relation, and id
  semantics needed to correlate those ledgers.

Do not rely on coincidentally equal `conversation_id` or `message_id` values
across ledgers as ownership. Cross-ledger causality must use explicit
`ledger_id`, `fact_id`, `message_id`, `content_id`, `revision_id`,
`correlation_id`, `causation_id`, and relation records.

## Core shape

`MessageFactEnvelope` is the immutable fact envelope for message-ledger
projection:

- `fact_id`: identity of one immutable ledger fact
- `fact_type`: what changed or was observed
- `ledger_id`: owner ledger namespace
- `message_id`: durable message identity inside the owner ledger
- `participant_id`: product/runtime participant identity
- `kind`: independent message-kind marker
- `occurred_at` and optional `observed_at`: event time and observation time
- optional `ledger_seq`: owner-ledger ordering projection
- optional `content_ref`: pointer to typed content
- optional `relation`: reply, revision, redaction, compact, tool-result, or
  other relation fact
- optional `projection_state`: current visibility/state hint for projections

`MessageCurrentProjection` is a read model for online current-state queries. It
is not the immutable source of truth.

## Message kinds

Message kinds are modeled independently from read/write and context-inclusion
policy.

Current core kinds:

- `text`
- `html`
- `asset`
- `audio`
- `video`
- `file`
- `tool_call`
- `tool_result`
- `thinking`
- `compact`
- `system`
- `extension`

Use `extension` plus a namespaced `extension` value when a consumer needs a new
kind before it becomes a shared core kind.

Do not infer that `thinking`, `tool_call`, `tool_result`, or `compact` are
context-visible, user-visible, writable, or read-only solely from their kind.
Those decisions belong to projection and assembly policy.

## Content references

Message facts carry `MessageContentRef`, not necessarily inline content.

Supported storage reference forms:

- `inline`: small protocol-shaped payloads
- `table`: typed content records in an owner store
- `object`: OSS/blob storage with key, optional bucket, and optional URI
- `external`: externally addressed content

Typed content records currently include:

- `TextContentRecord`
- `HtmlContentRecord`
- `BlobContentRecord`
- `ToolCallContentRecord`
- `ToolResultContentRecord`
- `ThinkingContentRecord`
- `CompactContentRecord`

Large binary or generated artifacts should use object storage references with
`mime_type`, `byte_size`, and `checksum` where available.

## Relations

`MessageRelation` expresses durable message links without forcing one table
shape:

- `reply_to`
- `quotes`
- `forwards`
- `revises`
- `redacts`
- `compacts`
- `tool_result_of`
- `references`
- `derived_from`
- `extension`

`MessageRangeRef` supports compact and range-based operations by referencing an
owner ledger, optional conversation, optional sequence range, and optional
explicit message ids.

## Storage guidance

The shared contract is columnar-ready but not columnar-only.

Recommended mapping:

- online truth: row-store or wide-row-friendly OLTP ledger plus current
  projections for permissions, unread, latest-message, edits, deletes, and
  idempotency
- columnar projection: append-friendly fact stream for scans, audit, compact
  input, model-context analytics, runtime introspection, and search/index
  feeding
- object storage: large audio, video, image, file, and generated artifacts

Do not make a columnar database the only synchronous IM truth until edit,
redaction, deletion, delivery, permission, unread, and conflict behavior are
proven against real product use.
