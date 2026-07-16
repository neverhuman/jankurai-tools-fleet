//! Integration tests for the score-diff projection.
//!
//! These exercise the public `build_diff_report` surface of `jankurai-fleet`
//! end to end against representative score-report JSON, proving the projection
//! computes deltas, cap changes, and new/resolved/carried findings without
//! reaching back into the core audit runner.

use std::path::Path;

use jankurai_fleet::score::build_diff_report;
use serde_json::json;

/// A base report at score 70 with two caps and one high finding, compared to a
/// head report at score 88 that removed one cap and resolved the finding.
#[test]
fn diff_report_tracks_score_delta_caps_and_resolved_findings() {
    let base = json!({
        "score": 70,
        "raw_score": 72,
        "caps_applied": ["no-deterministic-fast-lane", "missing-agent-readable-docs"],
        "findings": [
            {
                "rule_id": "HLT-004-UNMAPPED-PROOF",
                "severity": "high",
                "path": "agent/test-map.json",
                "problem": "path `Cargo.toml` has no test-map proof route"
            }
        ]
    });
    let head = json!({
        "score": 88,
        "raw_score": 88,
        "caps_applied": ["missing-agent-readable-docs"],
        "findings": []
    });

    let report = build_diff_report(
        Path::new("agent/baselines/main.repo-score.json"),
        Path::new("target/jankurai/repo-score.json"),
        &base,
        &head,
    );

    assert_eq!(report.base_score, 70);
    assert_eq!(report.head_score, 88);
    assert_eq!(report.score_delta, 18);
    assert_eq!(report.raw_score_delta, 16);

    // The fast-lane cap was lifted; the docs cap carried over.
    assert_eq!(
        report.caps_removed,
        vec!["no-deterministic-fast-lane".to_string()]
    );
    assert!(report.caps_added.is_empty());

    // The single high finding was resolved and nothing new appeared.
    assert_eq!(report.resolved_findings.len(), 1);
    assert!(report.new_findings.is_empty());
    assert!(report.carried_findings.is_empty());
    assert_eq!(report.resolved_high_or_critical, 1);
    assert_eq!(report.new_high_or_critical, 0);
}

/// A regression: the head report drops below the base and introduces a new
/// critical finding plus a new cap. The projection must surface the regression.
#[test]
fn diff_report_flags_regressions_and_new_caps() {
    let base = json!({
        "score": 90,
        "raw_score": 90,
        "caps_applied": [],
        "findings": []
    });
    let head = json!({
        "score": 64,
        "raw_score": 70,
        "caps_applied": ["no-security-lane-on-high-risk-repo"],
        "findings": [
            {
                "rule_id": "HLT-010-SECRET-SPRAWL",
                "severity": "critical",
                "path": "crates/jankurai-fleet/src/score.rs",
                "problem": "secret-like content detected"
            }
        ]
    });

    let report = build_diff_report(
        Path::new("agent/baselines/main.repo-score.json"),
        Path::new("target/jankurai/repo-score.json"),
        &base,
        &head,
    );

    assert_eq!(report.score_delta, -26);
    assert_eq!(
        report.caps_added,
        vec!["no-security-lane-on-high-risk-repo".to_string()]
    );
    assert!(report.caps_removed.is_empty());
    assert_eq!(report.new_findings.len(), 1);
    assert_eq!(report.new_high_or_critical, 1);
    assert_eq!(report.new_findings[0].severity, "critical");
}
