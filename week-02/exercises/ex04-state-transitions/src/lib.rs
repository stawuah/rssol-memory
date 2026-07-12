#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    ElectionTimeout,
    WonElection,
    ObservedHigherTermLeader,
}

pub fn next_role(current: Role, event: Event) -> Role {
    todo!("implement a small deterministic state transition table")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follower_times_out_into_candidate() {
        assert_eq!(next_role(Role::Follower, Event::ElectionTimeout), Role::Candidate);
    }

    #[test]
    fn candidate_wins_becomes_leader() {
        assert_eq!(next_role(Role::Candidate, Event::WonElection), Role::Leader);
    }

    #[test]
    fn higher_term_forces_follower() {
        assert_eq!(
            next_role(Role::Leader, Event::ObservedHigherTermLeader),
            Role::Follower
        );
    }
}
