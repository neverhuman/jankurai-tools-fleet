# Changelog

All notable changes to jankurai-tools-fleet are documented in this file. The
format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and
this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html). The authoritative
version string lives in [`VERSION`](VERSION).

## [Unreleased]

### Added

- Root `Justfile` command surface with `setup`, `fast`, `check`, `security`, and
  `audit` lanes for one-command setup and validation.
- GitHub Actions CI (`.github/workflows/ci.yml`) with build/fast, security
  (secret + dependency scanning), and jankurai audit jobs, all third-party
  actions pinned to commit SHAs.
- Pinned CI lane scripts under `ops/ci/` and a local entry point at
  `scripts/ci-local.sh` so local and CI runs execute the same commands.
- Agent-readable documentation: `README.md`, `docs/architecture.md`,
  `docs/testing.md`, `docs/boundaries.md`, `docs/release.md`, and
  `docs/exceptions.md`.
- `VERSION` and `rust-toolchain.toml` to pin the release version and toolchain.

### Changed

- Re-scoped `agent/boundaries.toml`, `agent/owner-map.json`,
  `agent/test-map.json`, `agent/generated-zones.toml`, and
  `agent/proof-lanes.toml` to the paths that exist in this single-purpose repo.

## [1.7.0-split.0] - 2026-06-12

### Added

- Initial split-family extraction of the jankurai-fleet projection crate
  (fleet matrix, score diff/trend, score history, diff-audit, repair-task bank).
