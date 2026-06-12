# jankurai-tools-fleet Architecture

jankurai-tools-fleet holds the fleet-family projection commands extracted from
`jankurai-core`: the fleet matrix, score diff/trend, score history, diff-audit,
and the repair-task bank. The product standard the family defines is:

```text
Rust core + TypeScript/React/Vite product surface + PostgreSQL truth
+ generated contracts + exception-only Python AI/data service
```

This repository is the Rust-only `jankurai-fleet` crate, so only the Rust arm of
the standard is exercised here. New implementation is Rust-first. Agents must not
write Python for repo tools, proof lanes, product services, general backend glue,
authorization, or production database writes. Python is allowed only for rare
advanced ML/data library work that has no practical Rust/TypeScript alternative,
stays boxed to a dated exception, and never lands in this crate.

## Crate layout

| Module | Role |
| --- | --- |
| `crates/jankurai-fleet/src/fleet.rs` | fleet matrix projection over per-repo scores |
| `crates/jankurai-fleet/src/score.rs` | score diff/trend value model |
| `crates/jankurai-fleet/src/score_history.rs` | bounded score-history ledger and mirror recovery |
| `crates/jankurai-fleet/src/history.rs` | history read/export/compact/restore surfaces |
| `crates/jankurai-fleet/src/diff_audit.rs` | diff-audit projection over changed paths |
| `crates/jankurai-fleet/src/repair_tasks.rs` | repair-task bank projection |

The crate composes the shared audit substrate (`jankurai-audit-kernel`) without
reimplementing scoring. Commands that need the core audit runner accept it as an
injected closure, so this crate never depends back on `jankurai-core`.

## Workspace ownership

| Path | Role |
| --- | --- |
| `crates/` | Rust fleet/score/history/diff/repair projection crate |
| `agent/` | machine-readable owner, test, boundary, and proof maps |
| `docs/` | architecture, testing, boundaries, release, and exception docs |
| `schemas/` | JSON Schemas for the fleet/score artifacts |
| `ops/` | pinned CI script entrypoints |
| `scripts/` | local CI and fusion helpers |

Agents should prefer `agent/owner-map.json` and `agent/test-map.json` for
changes, then route to the smallest proof lane in `agent/proof-lanes.toml`.
