/// This exercise is about reading Anchor-style lifetimes.
pub struct Context<'info, T> {
    pub accounts: &'info mut T,
}

pub struct Counter {
    pub value: u64,
}

pub fn bump_counter<'info>(ctx: Context<'info, Counter>) {
    // Coding map: follow ctx.accounts to its `value` field and mutate through
    // the exclusive reference. Production code should use checked arithmetic.
    todo!("mutate the borrowed account state")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_through_context() {
        let mut counter = Counter { value: 1 };
        let ctx = Context {
            accounts: &mut counter,
        };
        bump_counter(ctx);
        assert_eq!(counter.value, 2);
    }
}
