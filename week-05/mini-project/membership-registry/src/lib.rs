#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    pub node_id: String,
    pub last_tick: u64,
}

#[derive(Debug, Default)]
pub struct MembershipRegistry {
    members: Vec<Member>,
}

impl MembershipRegistry {
    pub fn update(&mut self, _node_id: &str, _tick: u64) {
        todo!("insert a new member or update last_tick if the node already exists")
    }

    pub fn get(&self, _node_id: &str) -> Option<&Member> {
        todo!("return borrowed member if present")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updates_existing_member() {
        let mut registry = MembershipRegistry::default();
        registry.update("node-a", 4);
        registry.update("node-a", 9);
        assert_eq!(registry.get("node-a").unwrap().last_tick, 9);
    }
}
