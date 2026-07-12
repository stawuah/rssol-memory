#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfidentialTransfer {
    pub amount_hidden: bool,
    pub timing_hidden: bool,
    pub intent_hidden: bool,
    pub proof_verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeakReport {
    pub visible: Vec<&'static str>,
    pub private: Vec<&'static str>,
    pub missing_controls: Vec<&'static str>,
}

/// Analogy:
/// Confidential transfer is a tinted glass window.
/// People cannot see the exact amount, but they can still see the car arrive, the time, and the direction.
pub fn analyze(flow: ConfidentialTransfer) -> LeakReport {
    // Coding map:
    // 1. Create visible, private, and missing_controls vectors.
    // 2. Classify amount, timing, and intent from their booleans.
    // 3. Add "batching" when timing is visible.
    // 4. Add a suitable intent-hiding control when intent is visible.
    // 5. Add "proof" when proof_verified is false.
    // 6. Return LeakReport containing all three vectors.
    todo!("classify what stays visible and what needs extra controls")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amount_can_be_hidden_while_timing_leaks() {
        let report = analyze(ConfidentialTransfer {
            amount_hidden: true,
            timing_hidden: false,
            intent_hidden: false,
            proof_verified: true,
        });

        assert!(report.private.contains(&"amount"));
        assert!(report.visible.contains(&"timing"));
        assert!(report.missing_controls.contains(&"batching"));
    }

    #[test]
    fn proof_is_required() {
        let report = analyze(ConfidentialTransfer {
            amount_hidden: true,
            timing_hidden: true,
            intent_hidden: true,
            proof_verified: false,
        });

        assert!(report.missing_controls.contains(&"proof"));
    }
}
