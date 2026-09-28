//! ex16 — Raft leader election: terms, one vote per term, up-to-date logs,
//! and stepping down.

use std::collections::BTreeSet;

pub type NodeId = u32;
pub type Term = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogPos {
    pub last_index: u64,
    pub last_term: Term,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestVote {
    pub term: Term,
    pub candidate: NodeId,
    pub last_log: LogPos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VoteReply {
    pub term: Term,
    pub from: NodeId,
    pub granted: bool,
}

pub struct RaftNode {
    pub id: NodeId,
    pub term: Term,
    pub voted_for: Option<NodeId>,
    pub role: Role,
    pub log: LogPos,
    cluster_size: usize,
    votes: BTreeSet<NodeId>,
}

/// Raft §5.4.1: the candidate's log is at least as up to date as ours if its
/// last term is higher, or the terms are equal and its log is at least as long.
pub fn log_is_up_to_date(candidate: LogPos, mine: LogPos) -> bool {
    todo!("compare last term, then last index")
}

impl RaftNode {
    pub fn new(id: NodeId, cluster_size: usize, log: LogPos) -> Self {
        RaftNode { id, term: 0, voted_for: None, role: Role::Follower, log, cluster_size, votes: BTreeSet::new() }
    }

    /// Seeing a higher term ALWAYS makes you a follower in that term.
    fn step_down(&mut self, term: Term) {
        self.term = term;
        self.voted_for = None;
        self.role = Role::Follower;
        self.votes.clear();
    }

    /// Election timeout fired: become a candidate in a new term.
    pub fn start_election(&mut self) -> RequestVote {
        // Coding map:
        // 1. term += 1, role = Candidate, voted_for = Some(self.id).
        // 2. votes = {self.id}. (A 1-node cluster wins immediately: check majority.)
        // 3. Return the RequestVote for this term.
        todo!("new term, vote for self, ask the others")
    }

    pub fn on_request_vote(&mut self, req: RequestVote) -> VoteReply {
        // Coding map:
        // 1. If req.term > self.term: step_down(req.term).
        // 2. Grant iff req.term == self.term
        //       && voted_for is None or Some(req.candidate)
        //       && log_is_up_to_date(req.last_log, self.log).
        // 3. If granted, set voted_for = Some(req.candidate).
        // 4. Reply with the CURRENT term (lets stale candidates step down).
        todo!("term check, one vote per term, log check")
    }

    pub fn on_vote_reply(&mut self, reply: VoteReply) -> Role {
        // Coding map:
        // 1. reply.term > self.term -> step_down(reply.term), return Follower.
        // 2. Ignore if not Candidate, reply.term != self.term, or not granted.
        // 3. Insert reply.from into votes (a set: duplicates don't double count).
        // 4. votes.len() > cluster_size / 2 -> Leader.
        todo!("count granted votes for the current term")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(i: u64, t: Term) -> LogPos {
        LogPos { last_index: i, last_term: t }
    }

    fn cluster(n: u32, log: LogPos) -> Vec<RaftNode> {
        (0..n).map(|id| RaftNode::new(id, n as usize, log)).collect()
    }

    #[test]
    fn up_to_date_rule() {
        assert!(log_is_up_to_date(pos(1, 3), pos(10, 2))); // higher term wins over length
        assert!(!log_is_up_to_date(pos(10, 2), pos(1, 3)));
        assert!(log_is_up_to_date(pos(5, 2), pos(5, 2)));
        assert!(!log_is_up_to_date(pos(4, 2), pos(5, 2)));
    }

    #[test]
    fn candidate_with_majority_becomes_leader() {
        let mut nodes = cluster(5, pos(3, 1));
        let req = nodes[0].start_election();
        let mut role = Role::Candidate;
        for i in 1..3 {
            let reply = nodes[i].on_request_vote(req);
            assert!(reply.granted);
            role = nodes[0].on_vote_reply(reply);
        }
        assert_eq!(role, Role::Leader);
        assert_eq!(nodes[0].term, 1);
    }

    #[test]
    fn one_vote_per_term() {
        let mut nodes = cluster(3, pos(0, 0));
        let a = nodes[0].start_election();
        let b = nodes[1].start_election();
        assert!(nodes[2].on_request_vote(a).granted);
        assert!(!nodes[2].on_request_vote(b).granted);
        assert!(nodes[2].on_request_vote(a).granted); // re-asking the same candidate is fine
    }

    #[test]
    fn stale_log_cannot_win() {
        let mut behind = RaftNode::new(0, 3, pos(2, 1));
        let mut ahead = RaftNode::new(1, 3, pos(5, 2));
        let req = behind.start_election();
        let reply = ahead.on_request_vote(req);
        assert!(!reply.granted);
        assert_eq!(ahead.term, 1); // still adopts the newer term
    }

    #[test]
    fn higher_term_forces_step_down_and_duplicates_do_not_count() {
        let mut n = RaftNode::new(0, 5, pos(0, 0));
        n.start_election();
        let dup = VoteReply { term: 1, from: 3, granted: true };
        n.on_vote_reply(dup);
        assert_eq!(n.on_vote_reply(dup), Role::Candidate); // 2 distinct votes of 5
        assert_eq!(n.on_vote_reply(VoteReply { term: 9, from: 4, granted: false }), Role::Follower);
        assert_eq!((n.term, n.voted_for), (9, None));
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("simulate a split vote in a 4-node cluster (2 candidates, 2+2): nobody wins; a new term resolves it")
    }
}
