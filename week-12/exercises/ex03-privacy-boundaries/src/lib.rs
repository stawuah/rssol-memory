#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivacySurface {
    pub amount_hidden: bool,
    pub timing_hidden: bool,
    pub intent_hidden: bool,
    pub account_addresses_public: bool,
    pub proof_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivacyReport {
    pub hidden: Vec<&'static str>,
    pub exposed: Vec<&'static str>,
    pub design_gap: Vec<&'static str>,
}

pub fn analyze_confidential_transfer(surface: PrivacySurface) -> PrivacyReport {
    let mut hidden = Vec::new();
    let mut exposed = Vec::new();
    let mut design_gap = Vec::new();

    if surface.amount_hidden {
        hidden.push("amount");
    } else {
        exposed.push("amount");
    }

    if surface.timing_hidden {
        hidden.push("timing");
    } else {
        exposed.push("timing");
        design_gap.push("batching_or_delay");
    }

    if surface.intent_hidden {
        hidden.push("intent");
    } else {
        exposed.push("intent");
        design_gap.push("intent_mitigation");
    }

    if surface.account_addresses_public {
        exposed.push("addresses");
    } else {
        hidden.push("addresses");
    }

    if !surface.proof_required {
        design_gap.push("onchain_verification");
    }

    PrivacyReport {
        hidden,
        exposed,
        design_gap,
    }
}

pub fn explain_privacy_boundary() -> &'static str {
    "Confidential transfer can hide value, but timing and intent usually need extra protocol design."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confidential_transfer_hides_amount_but_not_timing_or_intent() {
        let report = analyze_confidential_transfer(PrivacySurface {
            amount_hidden: true,
            timing_hidden: false,
            intent_hidden: false,
            account_addresses_public: true,
            proof_required: true,
        });

        assert!(report.hidden.contains(&"amount"));
        assert!(report.exposed.contains(&"timing"));
        assert!(report.exposed.contains(&"intent"));
        assert!(report.design_gap.contains(&"batching_or_delay"));
        assert!(report.design_gap.contains(&"intent_mitigation"));
    }

    #[test]
    fn missing_proof_is_a_gap() {
        let report = analyze_confidential_transfer(PrivacySurface {
            amount_hidden: true,
            timing_hidden: true,
            intent_hidden: true,
            account_addresses_public: true,
            proof_required: false,
        });

        assert!(report.design_gap.contains(&"onchain_verification"));
    }
}

