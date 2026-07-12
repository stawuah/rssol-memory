#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Created(String),
    Deleted(String),
}

pub fn apply_events(events: &[Event]) -> Vec<String> {
    todo!("build a simple read model of currently existing ids")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_current_view() {
        let events = [
            Event::Created("a".into()),
            Event::Created("b".into()),
            Event::Deleted("a".into()),
        ];
        assert_eq!(apply_events(&events), vec!["b".to_string()]);
    }
}
