//! ex03 — Mutex per account, atomic transfers, and deadlock-free lock ordering.

use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerError {
    UnknownAccount,
    SameAccount,
    InsufficientFunds,
    Overflow,
}

/// One lock per account, so transfers between unrelated accounts run in
/// parallel (like Solana's per-account write locks, in miniature).
pub struct Ledger {
    accounts: Vec<Mutex<u64>>,
}

impl Ledger {
    pub fn new(balances: &[u64]) -> Self {
        Ledger { accounts: balances.iter().map(|&b| Mutex::new(b)).collect() }
    }

    pub fn balance(&self, account: usize) -> Option<u64> {
        self.accounts.get(account).map(|m| *m.lock().expect("poisoned"))
    }

    /// Move `amount` from `from` to `to`. Either both balances change or
    /// neither does. Must never deadlock, whatever the order of arguments.
    pub fn transfer(&self, from: usize, to: usize, amount: u64) -> Result<(), LedgerError> {
        // Coding map:
        // 1. Reject from == to (locking the same Mutex twice = self-deadlock).
        // 2. Reject indices out of range.
        // 3. Lock the LOWER index first, then the higher. Every thread uses
        //    the same global order, so an A->B / B->A cycle can't form.
        // 4. Map the two guards back to (source, destination).
        // 5. Compute new balances with checked_sub / checked_add into temps.
        // 6. Only if both succeed, write both. Guards unlock on drop.
        todo!("lock in index order and move funds atomically")
    }

    /// Sum of all balances, as one consistent snapshot.
    pub fn total(&self) -> u128 {
        // Coding map:
        // 1. Lock ALL accounts in index order and keep the guards alive
        //    (collect them into a Vec) so no transfer runs mid-sum.
        // 2. Sum as u128.
        todo!("lock everything in order, then sum")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn transfer_moves_funds() {
        let l = Ledger::new(&[100, 0]);
        assert_eq!(l.transfer(0, 1, 40), Ok(()));
        assert_eq!((l.balance(0), l.balance(1)), (Some(60), Some(40)));
    }

    #[test]
    fn failures_change_nothing() {
        let l = Ledger::new(&[10, u64::MAX]);
        assert_eq!(l.transfer(0, 1, 11), Err(LedgerError::InsufficientFunds));
        assert_eq!(l.transfer(0, 1, 5), Err(LedgerError::Overflow));
        assert_eq!(l.transfer(0, 0, 1), Err(LedgerError::SameAccount));
        assert_eq!(l.transfer(0, 9, 1), Err(LedgerError::UnknownAccount));
        assert_eq!((l.balance(0), l.balance(1)), (Some(10), Some(u64::MAX)));
    }

    #[test]
    fn opposite_transfers_never_deadlock_and_conserve_money() {
        let (done_tx, done_rx) = mpsc::channel();
        std::thread::spawn(move || {
            let l = Ledger::new(&[1_000_000; 4]);
            std::thread::scope(|s| {
                for t in 0..8 {
                    let l = &l;
                    s.spawn(move || {
                        for i in 0..20_000 {
                            let (a, b) = if t % 2 == 0 { (i % 4, (i + 1) % 4) } else { ((i + 1) % 4, i % 4) };
                            let _ = l.transfer(a, b, 1);
                        }
                    });
                }
            });
            done_tx.send(l.total()).unwrap();
        });
        let total = match done_rx.recv_timeout(Duration::from_secs(20)) {
            Ok(total) => total,
            Err(mpsc::RecvTimeoutError::Timeout) => panic!("deadlock: transfers never finished"),
            Err(mpsc::RecvTimeoutError::Disconnected) => panic!("a transfer thread panicked (see output above)"),
        };
        assert_eq!(total, 4_000_000);
    }

    #[test]
    #[ignore = "learner must add an adversarial test"]
    fn learner_adversarial_test() {
        todo!("run total() concurrently with transfers and assert every snapshot equals the initial total")
    }
}
