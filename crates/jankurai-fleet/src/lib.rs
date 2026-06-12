//! jankurai fleet-family tools, extracted from `jankurai-core` (W5 split).
//!
//! This crate holds the projection/aggregation commands that compose the shared
//! audit substrate (`jankurai-audit-kernel`) without reimplementing scoring:
//! fleet matrix, score diff/trend, score history, diff-audit, and the
//! repair-task bank. Commands that need the core audit runner accept it as an
//! injected closure so this crate never depends back on `jankurai-core`.

pub mod diff_audit;
pub mod fleet;
pub mod history;
pub mod repair_tasks;
pub mod score;
pub mod score_history;
