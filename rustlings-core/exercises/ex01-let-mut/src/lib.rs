pub fn grow_counter(start: i32, step: i32) -> i32 {
    // Coding map:
    // 1. Copy `start` into a binding that is allowed to change.
    // 2. Apply `step` once.
    // 3. Make the final expression the value returned by the function.
    let mut counter = start;

    counter += step;
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_the_step() {
        assert_eq!(grow_counter(2, 3), 5);
    }

    #[test]
    fn handles_zero_step() {
        assert_eq!(grow_counter(10, 0), 10);
    }
}
