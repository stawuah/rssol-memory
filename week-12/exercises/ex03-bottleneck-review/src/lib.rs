pub fn first_bottleneck(load: u64, capacity: u64) -> &'static str {
    todo!("return `within capacity` if load <= capacity, otherwise `overloaded`")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_capacity() {
        assert_eq!(first_bottleneck(8, 10), "within capacity");
        assert_eq!(first_bottleneck(12, 10), "overloaded");
    }
}
