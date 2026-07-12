#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferRequest {
    pub sender: &'static str,
    pub recipient: &'static str,
    pub amount: u64,
    pub submitted_slot: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferProof {
    pub verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransferStage {
    IntentSubmitted,
    ProofAttached,
    Settled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateTransfer {
    request: TransferRequest,
    proof: Option<TransferProof>,
    stage: TransferStage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteError {
    ProofMissing,
    ProofInvalid,
    TooEarly,
    AlreadySettled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObserverView {
    pub sender: &'static str,
    pub recipient: &'static str,
    pub stage: TransferStage,
    pub amount_visible: bool,
}

impl PrivateTransfer {
    pub fn new(request: TransferRequest) -> Self {
        Self {
            request,
            proof: None,
            stage: TransferStage::IntentSubmitted,
        }
    }

    pub fn attach_proof(&mut self, proof: TransferProof) {
        self.proof = Some(proof);
        self.stage = TransferStage::ProofAttached;
    }

    pub fn settle(&mut self, current_slot: u64) -> Result<(), RouteError> {
        if self.stage == TransferStage::Settled {
            return Err(RouteError::AlreadySettled);
        }

        match &self.proof {
            None => Err(RouteError::ProofMissing),
            Some(proof) if !proof.verified => Err(RouteError::ProofInvalid),
            Some(_) if current_slot <= self.request.submitted_slot => Err(RouteError::TooEarly),
            Some(_) => {
                self.stage = TransferStage::Settled;
                Ok(())
            }
        }
    }

    pub fn observer_view(&self) -> ObserverView {
        ObserverView {
            sender: self.request.sender,
            recipient: self.request.recipient,
            stage: self.stage.clone(),
            amount_visible: matches!(self.stage, TransferStage::Settled),
        }
    }

    pub fn amount(&self) -> Option<u64> {
        if self.stage == TransferStage::Settled {
            Some(self.request.amount)
        } else {
            None
        }
    }
}

pub fn explain_capstone_target() -> &'static str {
    "The protocol should hide amount early, delay obvious timing signals, and only settle when a proof passes on chain."
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> TransferRequest {
        TransferRequest {
            sender: "alice",
            recipient: "bob",
            amount: 42,
            submitted_slot: 100,
        }
    }

    #[test]
    fn observer_cannot_see_amount_before_settlement() {
        let transfer = PrivateTransfer::new(request());
        let view = transfer.observer_view();
        assert!(!view.amount_visible);
        assert_eq!(transfer.amount(), None);
    }

    #[test]
    fn settlement_requires_verified_proof_and_time_progress() {
        let mut transfer = PrivateTransfer::new(request());
        transfer.attach_proof(TransferProof { verified: true });

        assert_eq!(transfer.settle(100), Err(RouteError::TooEarly));
        assert_eq!(transfer.settle(101), Ok(()));
        assert_eq!(transfer.amount(), Some(42));
    }

    #[test]
    fn invalid_proof_is_rejected() {
        let mut transfer = PrivateTransfer::new(request());
        transfer.attach_proof(TransferProof { verified: false });
        assert_eq!(transfer.settle(101), Err(RouteError::ProofInvalid));
    }
}
