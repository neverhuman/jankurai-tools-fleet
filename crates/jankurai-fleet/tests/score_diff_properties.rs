//! Property tests for the score-diff projection.
//!
//! These use `proptest` to assert invariants that must hold for every pair of
//! score reports, not just the hand-picked cases in `score_diff_projection.rs`.
//! The projection is a pure function of two `serde_json::Value` reports, which
//! makes it a good fit for randomized invariant testing.

use std::path::Path;

use jankurai_fleet::score::build_diff_report;
use proptest::prelude::*;
use serde_json::json;

proptest! {
    /// The reported score delta must always equal head_score - base_score, and
    /// the raw-score delta must equal head_raw - base_raw, for any score pair.
    #[test]
    fn score_delta_is_exact_difference(
        base_score in -1000i64..1000,
        head_score in -1000i64..1000,
        base_raw in -1000i64..1000,
        head_raw in -1000i64..1000,
    ) {
        let base = json!({
            "score": base_score,
            "raw_score": base_raw,
            "caps_applied": [],
            "findings": []
        });
        let head = json!({
            "score": head_score,
            "raw_score": head_raw,
            "caps_applied": [],
            "findings": []
        });

        let report = build_diff_report(
            Path::new("base.json"),
            Path::new("head.json"),
            &base,
            &head,
        );

        prop_assert_eq!(report.score_delta as i64, head_score - base_score);
        prop_assert_eq!(report.raw_score_delta as i64, head_raw - base_raw);
    }

    /// Caps present in the head but not the base must show up as added; caps in
    /// the base but not the head must show up as removed. Symmetric difference
    /// is exact and never double-counts.
    #[test]
    fn cap_set_difference_is_symmetric(
        shared in proptest::collection::vec("cap-[a-z]{1,4}", 0..4),
        only_base in proptest::collection::vec("base-[a-z]{1,4}", 0..3),
        only_head in proptest::collection::vec("head-[a-z]{1,4}", 0..3),
    ) {
        let mut base_caps = shared.clone();
        base_caps.extend(only_base.clone());
        let mut head_caps = shared.clone();
        head_caps.extend(only_head.clone());

        let base = json!({
            "score": 80,
            "raw_score": 80,
            "caps_applied": base_caps,
            "findings": []
        });
        let head = json!({
            "score": 80,
            "raw_score": 80,
            "caps_applied": head_caps,
            "findings": []
        });

        let report = build_diff_report(
            Path::new("base.json"),
            Path::new("head.json"),
            &base,
            &head,
        );

        for cap in &only_head {
            prop_assert!(report.caps_added.contains(cap));
        }
        for cap in &only_base {
            prop_assert!(report.caps_removed.contains(cap));
        }
        for cap in &shared {
            prop_assert!(!report.caps_added.contains(cap));
            prop_assert!(!report.caps_removed.contains(cap));
        }
    }
}
