# jankurai-tools-fleet Testing

Testing is routed proof. Agents should not guess which tests matter.

| Lane | Purpose |
| --- | --- |
| `required` | lightweight gate: workspace manifest resolves against the locked graph |
| `fast` | deterministic local proof for most edits (`cargo check` + workspace tests) |
| `security` | secrets, dependencies, SBOM/SCA, workflow lint |
| `audit` | jankurai repo score and hard-rule findings |
| `coverage-audit` | parse deterministic coverage and proof-gap artifacts |
| `copy-code` | exact and high-confidence duplicate source scan |
| `full` | release/merge gate (`just check`) |

Every lane command is mirrored by `ops/ci/<lane>.sh` and surfaced locally through
`scripts/ci-local.sh`, so a green local gate means a green CI run. The proof
routing for each owned path lives in `agent/test-map.json`; the runnable lane set
lives in `agent/proof-lanes.toml`.

## Rust proof routing

Changes under `crates/` route to `cargo test --workspace --locked`. The
`jankurai-fleet` crate carries unit and integration tests next to the
projection commands (fleet matrix, score diff/trend, score history, diff-audit,
repair-task bank) so behavior is proven, not assumed. Manifest and lockfile
changes route to `cargo check --workspace --locked`.

## Observability and repair evidence

When a proof lane fails, keep the next agent on the shortest possible rerun path.

- Emit structured errors instead of only free-form prose whenever the tool can do
  it. The fleet/score/history/diff commands return typed `anyhow` errors with
  context so the failing surface is named.
- Record the failing command, exit code, changed paths, artifact paths, and
  rerun command in the receipt.
- Prefer typed telemetry or JSON envelopes under `target/jankurai/` over ad hoc
  log spam. Score diff, score trend, and history exports all validate their JSON
  against the matching `schemas/*.schema.json` on write.
- Surface the repair hint, docs URL, and common fixes together so the next rerun
  is obvious.
- If the lane writes a receipt or summary, make it point at the exact proof
  command the next agent should trust.

Observability repairs should stay typed. The fleet projection records the source
report path, the freshness classification (`fresh`, `cached`, or `stale`), and the
reason a row was demoted, so a dashboard never renders an out-of-date score as
current without saying so.

### Repair-hint and receipt convention

Every failing lane emits a structured `repair_hint` with `common fixes`, a
`rerun command`, and a `docs_url`. A `phase completion receipt` records the
command, exit code, changed paths, artifacts, and the rerun command the next
agent should trust. This repair receipt guidance is the agent-friendly
exception/error pattern for this repo: typed `anyhow` errors carry context, and
the receipt points at the exact proof command to re-run instead of leaving
free-form logging in scope.

## Budgets, quotas, and stops

Paid work or unbounded work needs an explicit ceiling before it starts.

- State the budget in time, runner minutes, tokens, API calls, or dollars.
- State the quota or cap that will stop the run.
- State the kill switch or stop condition that aborts the work once the cap is
  hit.
- Capture evidence of the stop in the receipt so the next agent can tell a
  planned stop from a silent failure.
- Do not keep retrying a paid job after the cap is reached without a fresh
  approval receipt.

For this workspace:

- `just required` runs the lightweight manifest gate with no network and no spend.
- `just fast` writes a deterministic check + test snapshot with a bounded runtime
  budget (`timeout_seconds` in `agent/proof-lanes.toml`).
- `just security` invokes secret and dependency scanning; its evidence envelope is
  written under `target/jankurai/security/`.
- `just audit` writes `.jankurai/repo-score.json` and `.jankurai/repo-score.md`;
  these files are git-ignored and are not accepted ratchet baselines.
- Accepted ratchet and public badge baselines live under `agent/baselines/`. CI
  copies the reviewed baseline to `target/jankurai/accepted-baseline.json` before
  the final ratchet audit.
- `just check` runs format, lint, fast lane, security evidence, and the final
  score as the release/merge gate.

Receipts should record the command, exit code, changed paths, artifacts, and the
rerun command that the next agent should trust. Prefer structured errors,
telemetry, and repair receipts that tell the next agent where to rerun proof.

## Schema-first work

For new contract files under `schemas/`, add a Rust test that loads the JSON and
checks the required fields or references the contract chain. Keep that proof under
`cargo test --workspace --locked` so the schema stays machine-readable while the
CLI surface is still being planned.
