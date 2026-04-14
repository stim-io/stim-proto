# stim-proto

Shared protocol-contract repository for `stim.io` participants.

## Current purpose

This repo is the canonical home for shared peer, discovery, and message contracts consumed across `stim`, `stim-server`, and `santi`.

At cold start, it stays intentionally small:

- one Rust crate for Rust-side consumers
- one TypeScript package for `stim` and other JS/TS-side consumers
- explicit versioning from the start
- local-development-first consumption posture before formal publishing automation exists

## Current layout

- `crates/stim-proto/` — Rust shared contract crate
- `packages/stim-proto/` — TypeScript shared contract package

## Versioning posture

The current baseline uses explicit prerelease versions in both language surfaces.

For the first execution wave, local development may consume this repo through direct git/path attachment rather than formal package/crate publishing.

That bootstrap convenience must not blur canonical ownership: `stim-proto` remains the source of truth for the shared contracts.

## What this repo owns

- shared peer/discovery/message contract definitions
- compatibility versioning for those shared contracts
- language-level package/crate surfaces needed by participating repos

## What this repo does not own

- transport implementation
- registry implementation
- product UI/runtime behavior
- one specific carrier such as P2P
