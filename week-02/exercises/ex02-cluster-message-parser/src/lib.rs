#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClusterMessage<'a> {
    Heartbeat { node_id: &'a str, term: u64 },
    VoteRequest { node_id: &'a str, term: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidShape,
    InvalidTerm,
    UnknownKind,
}

pub fn parse_message(input: &str) -> Result<ClusterMessage<'_>, ParseError> {
    todo!("parse strings like `heartbeat node-a 7` and `vote node-b 8`")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_heartbeat() {
        assert_eq!(
            parse_message("heartbeat node-a 7"),
            Ok(ClusterMessage::Heartbeat {
                node_id: "node-a",
                term: 7,
            })
        );
    }

    #[test]
    fn parses_vote_request() {
        assert_eq!(
            parse_message("vote node-b 8"),
            Ok(ClusterMessage::VoteRequest {
                node_id: "node-b",
                term: 8,
            })
        );
    }

    #[test]
    fn rejects_unknown_message_kind() {
        assert_eq!(parse_message("ping node-a 3"), Err(ParseError::UnknownKind));
    }
}
