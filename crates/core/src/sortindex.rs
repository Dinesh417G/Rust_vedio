//! Fractional indexing for chapter and block ordering.
//!
//! Moving one item rewrites one row. We only renumber when neighbouring keys
//! have been bisected so many times that `f64` is about to run out of room.

/// Gap used when appending to the end of a list.
pub const STEP: f64 = 1024.0;
/// Once two neighbours are closer than this, the list is rebalanced.
pub const MIN_GAP: f64 = 1e-6;

/// The key for an item placed between `before` and `after`.
///
/// `None` for `before` means "at the very start", `None` for `after` means
/// "at the very end".
pub fn between(before: Option<f64>, after: Option<f64>) -> f64 {
    match (before, after) {
        (None, None) => STEP,
        (None, Some(a)) => a - STEP,
        (Some(b), None) => b + STEP,
        (Some(b), Some(a)) => (b + a) / 2.0,
    }
}

/// True when `keys` (already in list order) needs a rebalance pass.
pub fn needs_rebalance(keys: &[f64]) -> bool {
    keys.windows(2).any(|w| {
        let gap = w[1] - w[0];
        !gap.is_finite() || gap < MIN_GAP
    })
}

/// The canonical evenly-spaced keys for a list of `len` items.
pub fn rebalanced(len: usize) -> Vec<f64> {
    (1..=len).map(|i| i as f64 * STEP).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_and_prepend() {
        assert_eq!(between(None, None), 1024.0);
        assert_eq!(between(Some(1024.0), None), 2048.0);
        assert_eq!(between(None, Some(1024.0)), 0.0);
    }

    #[test]
    fn insert_bisects_the_gap() {
        assert_eq!(between(Some(1024.0), Some(2048.0)), 1536.0);
    }

    #[test]
    fn repeated_bisection_is_detected_before_it_breaks() {
        // Keep inserting immediately after the first item until the gap is unusable.
        let mut lo = 1024.0f64;
        let hi = 2048.0f64;
        let mut steps = 0;
        loop {
            let mid = between(Some(lo), Some(hi));
            if needs_rebalance(&[lo, mid]) {
                break;
            }
            lo = mid;
            steps += 1;
            assert!(steps < 200, "rebalance was never triggered");
        }
        assert!(
            steps > 20,
            "f64 should survive many bisections, got {steps}"
        );
    }

    #[test]
    fn rebalance_produces_the_documented_ladder() {
        assert_eq!(rebalanced(3), vec![1024.0, 2048.0, 3072.0]);
        assert!(!needs_rebalance(&rebalanced(50)));
    }

    #[test]
    fn healthy_list_is_left_alone() {
        assert!(!needs_rebalance(&[1024.0, 2048.0, 3072.0]));
        assert!(needs_rebalance(&[1024.0, 1024.0 + 1e-9]));
    }
}
