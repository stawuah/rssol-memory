#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketState {
    Uninitialized,
    Active { deposits: u64 },
    Paused { deposits: u64 },
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Initialize,
    Deposit(u64),
    Pause,
    Resume,
    Close,
}

pub fn transition(state: MarketState, action: Action) -> Result<MarketState, &'static str> {
    // Coding map:
    // 1. Match the pair `(state, action)`.
    // 2. Encode initialize, deposit, pause, resume, and empty-close cases.
    // 3. Preserve deposits across pause/resume.
    // 4. Use checked arithmetic for deposits.
    // 5. Reject non-empty close separately, then reject all other pairs.
    todo!("match every legal and illegal state transition")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follows_the_happy_path() {
        let state = transition(MarketState::Uninitialized, Action::Initialize).unwrap();
        let state = transition(state, Action::Deposit(10)).unwrap();
        let state = transition(state, Action::Pause).unwrap();
        assert_eq!(state, MarketState::Paused { deposits: 10 });
    }

    #[test]
    fn rejects_illegal_transitions() {
        assert_eq!(transition(MarketState::Closed, Action::Resume), Err("invalid transition"));
        assert_eq!(transition(MarketState::Active { deposits: 2 }, Action::Close), Err("market not empty"));
    }
}
