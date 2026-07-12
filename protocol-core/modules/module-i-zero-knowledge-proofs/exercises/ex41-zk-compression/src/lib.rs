/// A deliberately small policy model for Light-style ZK compression.
///
/// The exercise README explains how each field maps to real Solana accounts,
/// programs, proofs, timing, or operational evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Controls {
    pub identity_valid: bool,
    pub authority_valid: bool,
    pub state_valid: bool,
    pub within_limit: bool,
    pub value: u64,
    pub limit: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExerciseError {
    WrongIdentity,
    Unauthorized,
    InvalidState,
    LimitExceeded,
    AlreadyExecuted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Outcome { pub accepted_value: u64, pub executed: bool }

pub fn zk_compression(outcome: &mut Outcome, controls: Controls) -> Result<(), ExerciseError> {
    // Coding map:
    // 1. Validate identity/trust source before interpreting its data.
    // 2. Validate authority separately; identity is not authorization.
    // 3. Validate current state and reject replay when already executed.
    // 4. Validate the domain-specific bound and value <= limit.
    // 5. Calculate the new Outcome in a temporary value.
    // 6. Commit once only after every check succeeds.
    // 7. Replace each generic control with the concrete mechanism described in README.
    todo!("enforce the exercise invariant atomically")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid() -> Controls { Controls { identity_valid:true, authority_valid:true, state_valid:true, within_limit:true, value:10, limit:10 } }

    #[test]
    fn valid_transition_executes_once() {
        let mut out = Outcome::default();
        assert_eq!(zk_compression(&mut out, valid()), Ok(()));
        assert_eq!(out, Outcome { accepted_value:10, executed:true });
        assert_eq!(zk_compression(&mut out, valid()), Err(ExerciseError::AlreadyExecuted));
    }

    #[test]
    fn wrong_identity_fails_without_mutation() {
        let mut out = Outcome::default();
        let before = out;
        assert_eq!(zk_compression(&mut out, Controls { identity_valid:false, ..valid() }), Err(ExerciseError::WrongIdentity));
        assert_eq!(out, before);
    }

    #[test]
    fn over_limit_fails_without_mutation() {
        let mut out = Outcome::default();
        assert_eq!(zk_compression(&mut out, Controls { value:11, ..valid() }), Err(ExerciseError::LimitExceeded));
        assert_eq!(out, Outcome::default());
    }

    #[test]
    #[ignore = "learner must add a domain-specific adversarial test"]
    fn learner_adversarial_test() {
        todo!("model the README exploit or boundary and remove #[ignore]")
    }
}
