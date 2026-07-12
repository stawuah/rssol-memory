#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub target_program: &'static str,
    pub amount: u64,
}

/// Analogy:
/// CPI is one program asking another program to finish a subtask.
/// It is like handing a sealed request form to the back office.
pub fn route(call: Call) -> Result<&'static str, &'static str> {
    // Coding map:
    // 1. Reject call.amount == 0.
    // 2. Accept only target_program == "system".
    // 3. Return "sent" for that target and an error for every other target.
    todo!("forward the call with a small policy")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_rejected() {
        assert_eq!(
            route(Call {
                target_program: "system",
                amount: 0,
            }),
            Err("zero amount")
        );
    }

    #[test]
    fn system_route_succeeds() {
        assert_eq!(
            route(Call {
                target_program: "system",
                amount: 5,
            }),
            Ok("sent")
        );
    }
}
