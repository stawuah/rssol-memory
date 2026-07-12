#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeRole {
    Follower,
    Candidate,
    Leader,
}

pub fn can_start_election(role: NodeRole) -> bool {
    todo!("followers can start elections, leaders cannot")
}

pub fn can_accept_client_write(role: NodeRole) -> bool {
    todo!("only leaders accept client writes in this simplified model")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn election_rules() {
        assert!(can_start_election(NodeRole::Follower));
        assert!(can_start_election(NodeRole::Candidate));
        assert!(!can_start_election(NodeRole::Leader));
    }

    #[test]
    fn write_rules() {
        assert!(!can_accept_client_write(NodeRole::Follower));
        assert!(!can_accept_client_write(NodeRole::Candidate));
        assert!(can_accept_client_write(NodeRole::Leader));
    }
}
