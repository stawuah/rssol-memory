#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeState {
    pub node_id: String,
    pub role: Role,
    pub current_term: u64,
    pub voted_for: Option<String>,
    pub votes_received: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    ElectionTimeout,
    CastSelfVote,
    ReceiveVote,
    ObserveHigherTerm { term: u64, leader_id: String },
}

impl NodeState {
    pub fn new(node_id: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            role: Role::Follower,
            current_term: 0,
            voted_for: None,
            votes_received: 0,
        }
    }

    pub fn apply(&mut self, _event: Event) {
        todo!("update role, term, and votes in a deterministic way")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeout_starts_new_election() {
        let mut state = NodeState::new("node-a");
        state.apply(Event::ElectionTimeout);

        assert_eq!(state.role, Role::Candidate);
        assert_eq!(state.current_term, 1);
        assert_eq!(state.votes_received, 0);
    }

    #[test]
    fn self_vote_marks_voted_for_self() {
        let mut state = NodeState::new("node-a");
        state.apply(Event::ElectionTimeout);
        state.apply(Event::CastSelfVote);

        assert_eq!(state.voted_for.as_deref(), Some("node-a"));
        assert_eq!(state.votes_received, 1);
    }

    #[test]
    fn receiving_third_vote_becomes_leader() {
        let mut state = NodeState::new("node-a");
        state.apply(Event::ElectionTimeout);
        state.apply(Event::CastSelfVote);
        state.apply(Event::ReceiveVote);
        state.apply(Event::ReceiveVote);

        assert_eq!(state.role, Role::Leader);
    }

    #[test]
    fn higher_term_resets_to_follower() {
        let mut state = NodeState::new("node-a");
        state.apply(Event::ElectionTimeout);
        state.apply(Event::ObserveHigherTerm {
            term: 4,
            leader_id: "node-b".into(),
        });

        assert_eq!(state.role, Role::Follower);
        assert_eq!(state.current_term, 4);
        assert_eq!(state.voted_for, None);
        assert_eq!(state.votes_received, 0);
    }
}
