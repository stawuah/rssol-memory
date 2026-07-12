pub fn queue_growth_rate(produced: usize, consumed: usize) -> isize {
    todo!("return positive growth when produced > consumed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_growth_and_drain() {
        assert_eq!(queue_growth_rate(10, 6), 4);
        assert_eq!(queue_growth_rate(6, 10), -4);
    }
}
