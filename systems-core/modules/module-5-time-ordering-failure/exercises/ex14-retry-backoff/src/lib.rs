//! ex14 — Retries done right: exponential backoff, full jitter, budgets,
//! and only retrying what is safe to retry.

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    pub base: Duration,
    pub cap: Duration,
    /// Total attempts including the first (>= 1).
    pub max_attempts: u32,
    /// Stop retrying once the sum of sleeps would exceed this.
    pub budget: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetryError<E> {
    /// A non-retryable error: returned immediately.
    Permanent(E),
    /// Ran out of attempts; the last error.
    Exhausted(E),
    /// The next sleep would exceed the time budget; the last error.
    OverBudget(E),
}

/// Delay before retry number `attempt` (0-based), with "full jitter":
/// ceiling = min(cap, base * 2^attempt); delay = ceiling * jitter_permille / 1000.
/// `jitter_permille` is a random number in 0..=1000 supplied by the caller.
pub fn backoff_delay(base: Duration, cap: Duration, attempt: u32, jitter_permille: u32) -> Duration {
    // Coding map:
    // 1. factor = 2^attempt with checked_pow / checked_shl; on overflow use cap.
    // 2. ceiling = base.checked_mul(factor).unwrap_or(cap).min(cap).
    // 3. j = jitter_permille.min(1000); return ceiling * j / 1000.
    todo!("exponential ceiling, capped, scaled by jitter")
}

/// Call `op(attempt)` until it succeeds, fails permanently, runs out of
/// attempts, or the next sleep would exceed the budget.
/// `sleep` and `rng` are injected so tests are deterministic and instant.
pub fn retry<T, E>(
    policy: &RetryPolicy,
    mut op: impl FnMut(u32) -> Result<T, E>,
    is_retryable: impl Fn(&E) -> bool,
    mut sleep: impl FnMut(Duration),
    mut rng: impl FnMut() -> u32,
) -> Result<T, RetryError<E>> {
    // Coding map:
    // 1. assert!(policy.max_attempts >= 1).
    // 2. slept = ZERO. For attempt in 0..max_attempts:
    //    a. match op(attempt): Ok -> return Ok.
    //    b. Err(e) not retryable -> Permanent(e).
    //    c. Last attempt -> Exhausted(e).
    //    d. d = backoff_delay(base, cap, attempt, rng() % 1001).
    //       If slept + d > budget -> OverBudget(e).
    //    e. sleep(d); slept += d.
    todo!("retry loop with backoff, jitter and budget")
}

#[cfg(test)]
mod tests {
    use super::*;

    const MS: Duration = Duration::from_millis(1);

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum RpcError {
        Timeout,
        InvalidSignature,
    }

    fn policy() -> RetryPolicy {
        RetryPolicy { base: 100 * MS, cap: 2_000 * MS, max_attempts: 6, budget: 10_000 * MS }
    }

    #[test]
    fn backoff_grows_and_caps() {
        assert_eq!(backoff_delay(100 * MS, 2_000 * MS, 0, 1000), 100 * MS);
        assert_eq!(backoff_delay(100 * MS, 2_000 * MS, 3, 1000), 800 * MS);
        assert_eq!(backoff_delay(100 * MS, 2_000 * MS, 10, 1000), 2_000 * MS);
        assert_eq!(backoff_delay(100 * MS, 2_000 * MS, 200, 1000), 2_000 * MS); // no overflow
        assert_eq!(backoff_delay(100 * MS, 2_000 * MS, 3, 500), 400 * MS);
        assert_eq!(backoff_delay(100 * MS, 2_000 * MS, 3, 0), Duration::ZERO);
    }

    #[test]
    fn succeeds_after_transient_failures() {
        let mut sleeps = Vec::new();
        let r = retry(&policy(), |a| if a < 2 { Err(RpcError::Timeout) } else { Ok(a) }, |e| *e == RpcError::Timeout, |d| sleeps.push(d), || 1000);
        assert_eq!(r, Ok(2));
        assert_eq!(sleeps, vec![100 * MS, 200 * MS]);
    }

    #[test]
    fn permanent_errors_are_not_retried() {
        let mut calls = 0;
        let r: Result<(), _> = retry(&policy(), |_| { calls += 1; Err(RpcError::InvalidSignature) }, |e| *e == RpcError::Timeout, |_| {}, || 1000);
        assert_eq!(r, Err(RetryError::Permanent(RpcError::InvalidSignature)));
        assert_eq!(calls, 1);
    }

    #[test]
    fn exhausted_and_over_budget() {
        let mut calls = 0;
        let r: Result<(), _> = retry(&policy(), |_| { calls += 1; Err(RpcError::Timeout) }, |_| true, |_| {}, || 1000);
        assert_eq!(r, Err(RetryError::Exhausted(RpcError::Timeout)));
        assert_eq!(calls, 6);

        let tight = RetryPolicy { budget: 250 * MS, ..policy() };
        let mut slept = Vec::new();
        let r: Result<(), _> = retry(&tight, |_| Err(RpcError::Timeout), |_| true, |d| slept.push(d), || 1000);
        assert_eq!(r, Err(RetryError::OverBudget(RpcError::Timeout)));
        assert_eq!(slept, vec![100 * MS]); // 100 + 200 > 250
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("with rng returning random values, assert every sleep <= cap and total sleep <= budget")
    }
}
