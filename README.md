# stim-proto

Shared protocol-contract repository for `stim.io` participants.

## Current purpose

This repo is the canonical home for shared peer, discovery, and message contracts consumed across `stim`, `stim-server`, and `santi`.

At cold start, it stays intentionally small:

- one Rust crate for Rust-side consumers
- one TypeScript package for `stim` and other JS/TS-side consumers
- explicit versioning from the start
- Rust-side consumption through Git revisions rather than published Rust release artifacts
- npm package publication for TypeScript consumers through the dispatch-only publish workflow

## Current layout

- `crates/stim-proto/` — Rust shared contract crate
- `packages/stim-proto/` — TypeScript shared contract package

## Versioning posture

The current baseline uses explicit prerelease versions in both language surfaces.

For the first execution wave, Rust consumers should use Git revisions or local path overrides. Do not publish Rust crate artifacts until there is a concrete external distribution need.

The TypeScript package may be published to GitHub Packages with `.github/workflows/publish-npm.yml`; that workflow builds and uploads the npm package artifact before publishing.

That bootstrap convenience must not blur canonical ownership: `stim-proto` remains the source of truth for the shared contracts.

## What this repo owns

- shared peer/discovery/message contract definitions
- compatibility versioning for those shared contracts
- language-level package/crate surfaces needed by participating repos
- npm artifact publication for the TypeScript package
- durable shared semantics that need to survive real architecture differences across `stim`, `stim-server`, and `santi`

## What this repo does not own

- transport implementation
- registry implementation
- product UI/runtime behavior
- one specific carrier such as P2P
- Rust crate release publication or target-matrix Rust build artifacts at the current stage

## Architecture-difference rule

`stim-proto` should absorb real shared semantic differences between consumers when those differences belong in the protocol boundary.

But it should not preserve every existing mismatch. If a difference only forces bridge code, duplicate paths, or distorted protocol shapes, the right move is to simplify the architecture rather than encode that mismatch as durable protocol surface.
