# AGENTS

## Purpose

This file manages two things only:

- the stable role of `stim-proto/` as the neutral shared protocol-contract boundary for `stim.io`
- the small set of durable cold-start rules that should remain stable while the first protocol surface lands

Detailed protocol design belongs in repo docs once the shared contract surface becomes real.

## Core Constraints

- `stim-proto/` owns shared peer, discovery, and message contract definitions; it must not drift into transport implementation, registry implementation, or product/runtime behavior.
- `stim-proto/` should absorb durable shared semantics that arise from real architecture differences across consumers, but it should not fossilize low-value mismatches that only create bridge code, duplicate paths, or protocol distortion.
- Keep the initial split minimal: one Rust crate and one TypeScript package unless a real consumer pressure requires further decomposition.
- Keep local cross-repo development practical from the start, but do not hide canonical ownership behind install-time magic.
- Keep versioning explicit now even while formal publish/release automation remains deferred.
- Avoid fake maturity: only add docs, workflows, or package structure that support the real first execution wave.

## Git / CI Baseline

- `main` should advance through PRs rather than direct pushes.
- Keep force-push protection and branch-deletion protection enabled for `main`.
- Keep squash merge as the default history strategy.
- Keep required green checks in front of merge once `.github/workflows/ci.yml` is active.

## Common Commands

- Run Rust checks: `cargo test`
- Install JS dependencies: `pnpm install`
- Typecheck TS package: `pnpm -C packages/stim-proto typecheck`

## Key File Index

- `AGENTS.md`: stable repo boundary and baseline rules
- `README.md`: repo purpose and local-development posture
- `Cargo.toml`: Rust workspace root
- `package.json`: JS workspace root
- `.github/workflows/ci.yml`: minimal executable CI baseline

## Update Rules

- Keep this file short and durable.
- Add fuller docs only when the shared contract surface becomes real enough to justify canonical repo docs.
- If bootstrap changes affect root workspace ownership or attachment, update the root repo docs there rather than smearing that responsibility into this repo.
