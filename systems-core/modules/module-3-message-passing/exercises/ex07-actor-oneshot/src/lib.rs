//! ex07 — The actor pattern: one thread owns the state, others send commands
//! and wait for a one-shot reply.

use std::collections::HashMap;
use std::sync::mpsc::{self, SyncSender};
use std::thread::JoinHandle;

pub type AccountId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BankError {
    InsufficientFunds,
    Overflow,
    /// The actor has stopped; nobody will answer.
    ActorGone,
}

pub enum Command {
    Deposit { account: AccountId, amount: u64, reply: SyncSender<Result<u64, BankError>> },
    Withdraw { account: AccountId, amount: u64, reply: SyncSender<Result<u64, BankError>> },
    Balance { account: AccountId, reply: SyncSender<u64> },
    /// Stop processing; the actor thread returns its final state.
    Stop,
}

/// Apply one command to the state. Returns false if the actor should stop.
/// This is pure logic: no threads, easy to test.
pub fn handle_command(state: &mut HashMap<AccountId, u64>, cmd: Command) -> bool {
    // Coding map:
    // 1. Deposit: checked_add into the entry (default 0); reply Ok(new balance)
    //    or Err(Overflow) without changing state.
    // 2. Withdraw: checked_sub; reply Ok(new balance) or Err(InsufficientFunds).
    // 3. Balance: reply with the balance (0 if unknown).
    // 4. Ignore send errors on reply: the asker may have given up (timed out).
    // 5. Stop -> return false. Everything else -> true.
    todo!("apply the command and reply")
}

/// What callers hold. Cloning it is cheap (it's a Sender).
#[derive(Clone)]
pub struct BankHandle {
    tx: SyncSender<Command>,
}

/// Start the actor thread with a bounded mailbox. The JoinHandle yields the
/// final state when the actor stops (Stop command, or every handle dropped).
pub fn spawn_bank(mailbox: usize) -> (BankHandle, JoinHandle<HashMap<AccountId, u64>>) {
    // Coding map:
    // 1. (tx, rx) = mpsc::sync_channel(mailbox).
    // 2. Spawn a thread that OWNS a HashMap and loops over rx, calling
    //    handle_command until it returns false or the channel closes.
    // 3. Return the state from the thread.
    todo!("spawn the actor thread that owns the state")
}

impl BankHandle {
    /// Send a command built around a fresh one-shot reply channel and wait
    /// for the answer. Any channel failure means the actor is gone.
    fn ask<R>(&self, make: impl FnOnce(SyncSender<R>) -> Command) -> Result<R, BankError> {
        // Coding map:
        // 1. (reply_tx, reply_rx) = mpsc::sync_channel(1): room for exactly one answer.
        // 2. self.tx.send(make(reply_tx)); Err -> ActorGone.
        // 3. reply_rx.recv(); Err (actor dropped the reply sender) -> ActorGone.
        todo!("request/response over a one-shot channel")
    }

    pub fn deposit(&self, account: AccountId, amount: u64) -> Result<u64, BankError> {
        self.ask(|reply| Command::Deposit { account, amount, reply })?
    }

    pub fn withdraw(&self, account: AccountId, amount: u64) -> Result<u64, BankError> {
        self.ask(|reply| Command::Withdraw { account, amount, reply })?
    }

    pub fn balance(&self, account: AccountId) -> Result<u64, BankError> {
        self.ask(|reply| Command::Balance { account, reply })
    }

    /// Ask the actor to stop. Other handles will then see ActorGone.
    pub fn stop(&self) {
        let _ = self.tx.send(Command::Stop);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_operations() {
        let (bank, actor) = spawn_bank(8);
        assert_eq!(bank.deposit(1, 100), Ok(100));
        assert_eq!(bank.withdraw(1, 30), Ok(70));
        assert_eq!(bank.withdraw(1, 71), Err(BankError::InsufficientFunds));
        assert_eq!(bank.deposit(2, u64::MAX), Ok(u64::MAX));
        assert_eq!(bank.deposit(2, 1), Err(BankError::Overflow));
        assert_eq!(bank.balance(9), Ok(0));
        drop(bank);
        let state = actor.join().unwrap();
        assert_eq!(state[&1], 70);
        assert_eq!(state[&2], u64::MAX);
    }

    #[test]
    fn concurrent_withdrawals_never_overdraw() {
        let (bank, actor) = spawn_bank(4);
        bank.deposit(7, 100).unwrap();
        let successes = std::thread::scope(|s| {
            let hs: Vec<_> = (0..25)
                .map(|_| {
                    let bank = bank.clone();
                    s.spawn(move || bank.withdraw(7, 10).is_ok())
                })
                .collect();
            hs.into_iter().map(|h| h.join().unwrap()).filter(|&ok| ok).count()
        });
        assert_eq!(successes, 10);
        assert_eq!(bank.balance(7), Ok(0));
        drop(bank);
        actor.join().unwrap();
    }

    #[test]
    fn stopped_actor_reports_gone() {
        let (bank, actor) = spawn_bank(4);
        let other = bank.clone();
        bank.deposit(1, 5).unwrap();
        bank.stop();
        let state = actor.join().unwrap();
        assert_eq!(state[&1], 5);
        assert_eq!(other.balance(1), Err(BankError::ActorGone));
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("call handle_command directly with a Deposit that overflows and assert state is unchanged")
    }
}
