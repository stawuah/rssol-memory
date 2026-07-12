#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    LeaderElected,
    RequestFailed,
    HeartbeatSeen,
}

pub fn failed_events(events: &[Event]) -> Vec<Event> {
    todo!("use iterators to collect only failed events")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_only_failures() {
        let events = [
            Event::HeartbeatSeen,
            Event::RequestFailed,
            Event::LeaderElected,
            Event::RequestFailed,
        ];
        assert_eq!(
            failed_events(&events),
            vec![Event::RequestFailed, Event::RequestFailed]
        );
    }
}
