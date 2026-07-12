pub trait HasAmount {
    fn amount(&self) -> u64;
}

pub fn largest<T: HasAmount>(items: &[T]) -> Option<&T> {
    // Coding map: iterate by reference and select the item with maximum `amount()`.
    // Empty input must naturally produce None; do not clone or allocate.
    todo!("return a borrowed item with the greatest amount")
}

pub fn all_within_limit<T: HasAmount>(items: &[T], limit: u64) -> bool {
    // Coding map: use a short-circuiting iterator predicate over `amount()` values.
    todo!("use the trait bound without allocating")
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Order(u64);

    impl HasAmount for Order {
        fn amount(&self) -> u64 { self.0 }
    }

    #[test]
    fn generic_functions_work_for_trait_implementors() {
        let orders = [Order(3), Order(9), Order(4)];
        assert_eq!(largest(&orders).map(HasAmount::amount), Some(9));
        assert!(all_within_limit(&orders, 9));
        assert!(!all_within_limit(&orders, 8));
    }
}
