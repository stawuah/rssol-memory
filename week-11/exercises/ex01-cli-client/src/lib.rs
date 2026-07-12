pub fn build_increment_command(account: &str) -> String {
    todo!("return a human-readable mock client command for incrementing an account")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_command_string() {
        assert_eq!(build_increment_command("counter-1"), "increment counter-1");
    }
}
