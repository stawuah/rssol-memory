//! The ACTOR pattern: one thread OWNS some state; everyone else sends it
//! messages and gets answers back on a oneshot.
//! Run: cargo run --example actor
//!
//! Like you're five: the bank has ONE teller who alone touches the money
//! drawer. Customers hand over slips (messages) and wait for a receipt
//! (oneshot reply). No two people ever grab the drawer at once, so no locks.
//!
//! This is how most tokio services are structured (a "handle" + a task).
use std::collections::HashMap;
use std::time::Duration;
use strand::*;

enum Msg {
    Deposit { account: String, amount: u64 },
    Withdraw { account: String, amount: u64, reply: OneshotSender<Result<u64, String>> },
    Balance { account: String, reply: OneshotSender<u64> },
}

/// What callers hold. Cheap to clone; it's just a Sender.
#[derive(Clone)]
struct Bank {
    inbox: Sender<Msg>,
}

impl Bank {
    fn start() -> (Bank, std::thread::JoinHandle<()>) {
        let (inbox, rx) = bounded::<Msg>(128);
        let teller = std::thread::spawn(move || {
            let mut accounts: HashMap<String, u64> = HashMap::new(); // owned by THIS thread only
            for msg in rx {
                match msg {
                    Msg::Deposit { account, amount } => *accounts.entry(account).or_default() += amount,
                    Msg::Withdraw { account, amount, reply } => {
                        let bal = accounts.entry(account).or_default();
                        let result = if *bal >= amount {
                            *bal -= amount;
                            Ok(*bal)
                        } else {
                            Err(format!("insufficient funds: have {bal}, want {amount}"))
                        };
                        let _ = reply.send(result);
                    }
                    Msg::Balance { account, reply } => {
                        let _ = reply.send(accounts.get(&account).copied().unwrap_or(0));
                    }
                }
            }
            println!("teller: all handles dropped, closing the bank");
        });
        (Bank { inbox }, teller)
    }

    fn deposit(&self, account: &str, amount: u64) {
        let _ = self.inbox.send(Msg::Deposit { account: account.into(), amount });
    }

    fn withdraw(&self, account: &str, amount: u64) -> Result<u64, String> {
        let (reply, answer) = oneshot();
        let _ = self.inbox.send(Msg::Withdraw { account: account.into(), amount, reply });
        // Always put a deadline on a request to another component.
        answer.recv_timeout(Duration::from_secs(1)).map_err(|e| format!("no answer: {e:?}"))?
    }

    fn balance(&self, account: &str) -> u64 {
        let (reply, answer) = oneshot();
        let _ = self.inbox.send(Msg::Balance { account: account.into(), reply });
        answer.recv().unwrap_or(0)
    }
}

impl Share for Bank {
    type Handle = Bank;
    fn share(&self) -> Bank {
        self.clone()
    }
}

fn main() {
    let (bank, teller) = Bank::start();
    bank.deposit("kofi", 100);

    // 10 customers each try to withdraw 15. Only 6 can succeed (6 × 15 = 90).
    let results = spawn!(10, [bank], |id| {
        (id, bank.withdraw("kofi", 15))
    })
    .wait();
    let ok = results.iter().filter(|(_, r)| r.is_ok()).count();
    for (id, r) in &results {
        println!("customer {id}: {r:?}");
    }
    println!("{ok} succeeded, balance now {}", bank.balance("kofi"));
    assert_eq!(bank.balance("kofi"), 10);

    drop(bank); // last handle -> teller's loop ends -> thread exits
    teller.join().unwrap();
}
