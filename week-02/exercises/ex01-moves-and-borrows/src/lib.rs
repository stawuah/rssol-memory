#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub term: u64,
}

pub fn highest_term_node<'a>(nodes: &'a [Node]) -> Option<&'a Node> {
    todo!("return a shared reference to the node with the highest term")
}

pub fn promote_term(node: &mut Node) {
    todo!("increment the node term in place")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_reference_to_highest_term() {
        let nodes = vec![
            Node {
                id: "a".into(),
                term: 2,
            },
            Node {
                id: "b".into(),
                term: 5,
            },
        ];

        let winner = highest_term_node(&nodes).unwrap();
        assert_eq!(winner.id, "b");
        assert_eq!(winner.term, 5);
    }

    #[test]
    fn promotes_node_in_place() {
        let mut node = Node {
            id: "leader".into(),
            term: 7,
        };
        promote_term(&mut node);
        assert_eq!(node.term, 8);
    }
}
