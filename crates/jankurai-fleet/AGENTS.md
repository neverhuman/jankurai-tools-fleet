# Jankurai Tools Fleet Crate

## Workspace Boundary

- Work only in the user-named active repo/worktree.
- Never switch to sibling clones, archives, backups, resolved symlink targets, `/tmp` worktrees, or duplicate roots.
- Never create repo copies or side folders outside the active repo; preserve work with git branches.
- Before edits, report `pwd`, `git rev-parse --show-toplevel`, and `git status --short --branch`.
- Use Jeryu APIs/CLI for local GitLab/MR work; no `glab`, credential scraping, or raw local GitLab API calls.

Read the root `AGENTS.md`, `SPLIT.md`, and `agent/JANKURAI_STANDARD.md` first.

This crate owns the fleet score-history, score-diff projection, and repair-task
tooling that consume the auditor's `repo-score.json` and `repair-queue.jsonl`
artifacts. Keep the fleet tooling Rust-first; do not add Python helpers for
fleet behavior.

For score-history or score-diff changes, add focused Rust tests under
`crates/jankurai-fleet/tests/`, then run:

```bash
cargo test -p jankurai-fleet
cargo check --workspace --locked
```

Do not hand-edit generated artifacts listed in `agent/generated-zones.toml`.
Regenerate score outputs with the root `just audit` lane.
