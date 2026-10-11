//! ex23 — The actor pattern in tokio: mpsc mailbox, oneshot replies,
//! select! for shutdown, and timeouts on every request.

use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KvError {
    /// The actor stopped; nobody will answer.
    ActorGone,
    /// No answer within the handle's timeout (the actor may be overloaded).
    Timeout,
}

pub enum Cmd {
    Get { key: String, reply: oneshot::Sender<Option<u64>> },
    /// Returns the previous value.
    Set { key: String, value: u64, reply: oneshot::Sender<Option<u64>> },
    /// Test helper: make the actor busy for a while (simulates a slow command).
    Stall(Duration),
}

/// Clone-able client side.
#[derive(Clone)]
pub struct KvHandle {
    tx: mpsc::Sender<Cmd>,
    timeout: Duration,
}

/// Owner side: can shut the actor down and collect its final state.
pub struct KvService {
    handle: KvHandle,
    shutdown: oneshot::Sender<()>,
    task: JoinHandle<HashMap<String, u64>>,
}

/// The actor loop. Owns the map. Exits on shutdown OR when every handle is dropped.
pub async fn run_actor(mut rx: mpsc::Receiver<Cmd>, mut shutdown: oneshot::Receiver<()>) -> HashMap<String, u64> {
    // Coding map:
    // 1. let mut state = HashMap::new();
    // 2. loop { tokio::select! {
    //        biased;                                  // check shutdown first
    //        _ = &mut shutdown => break,              // Ok(()) or sender dropped: stop
    //        cmd = rx.recv() => match cmd {
    //            None => break,                       // all handles dropped
    //            Some(Cmd::Get{..}) => reply with state.get(&key).copied()
    //            Some(Cmd::Set{..}) => reply with state.insert(key, value)
    //            Some(Cmd::Stall(d)) => tokio::time::sleep(d).await
    //        }
    //    } }
    // 3. Ignore reply send errors (the asker may have timed out).
    // 4. Return state.
    todo!("select! over shutdown and the mailbox")
}

/// Start the actor on the current tokio runtime.
pub fn spawn_kv(mailbox: usize, timeout: Duration) -> KvService {
    // Coding map: mpsc::channel(mailbox), oneshot::channel(), tokio::spawn(run_actor(..)).
    todo!("create channels and spawn the actor task")
}

impl KvService {
    pub fn handle(&self) -> KvHandle {
        self.handle.clone()
    }

    /// Stop the actor and return its final state.
    pub async fn shutdown(self) -> HashMap<String, u64> {
        // Coding map: send on the shutdown oneshot (ignore error), then await the task.
        todo!("signal and await the actor")
    }
}

impl KvHandle {
    /// Send a command carrying a fresh oneshot and await the reply, all under
    /// ONE timeout (a full mailbox counts against the deadline too).
    async fn ask<R>(&self, make: impl FnOnce(oneshot::Sender<R>) -> Cmd) -> Result<R, KvError> {
        // Coding map:
        // 1. let (reply_tx, reply_rx) = oneshot::channel();
        // 2. tokio::time::timeout(self.timeout, async {
        //        self.tx.send(make(reply_tx)).await.map_err(|_| KvError::ActorGone)?;
        //        reply_rx.await.map_err(|_| KvError::ActorGone)
        //    }).await
        // 3. Elapsed -> Timeout; otherwise the inner Result.
        todo!("request/response with one deadline")
    }

    pub async fn get(&self, key: &str) -> Result<Option<u64>, KvError> {
        self.ask(|reply| Cmd::Get { key: key.to_string(), reply }).await
    }

    pub async fn set(&self, key: &str, value: u64) -> Result<Option<u64>, KvError> {
        self.ask(|reply| Cmd::Set { key: key.to_string(), value, reply }).await
    }

    /// Fire-and-forget: make the actor busy (for tests).
    pub async fn stall(&self, d: Duration) {
        let _ = self.tx.send(Cmd::Stall(d)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_and_set() {
        let svc = spawn_kv(16, Duration::from_secs(1));
        let kv = svc.handle();
        assert_eq!(kv.get("slot").await, Ok(None));
        assert_eq!(kv.set("slot", 10).await, Ok(None));
        assert_eq!(kv.set("slot", 11).await, Ok(Some(10)));
        assert_eq!(kv.get("slot").await, Ok(Some(11)));
        let state = svc.shutdown().await;
        assert_eq!(state["slot"], 11);
        assert_eq!(kv.get("slot").await, Err(KvError::ActorGone));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn many_concurrent_clients() {
        let svc = spawn_kv(8, Duration::from_secs(5));
        let mut tasks = tokio::task::JoinSet::new();
        for i in 0..100u64 {
            let kv = svc.handle();
            tasks.spawn(async move { kv.set(&format!("k{i}"), i).await });
        }
        while let Some(r) = tasks.join_next().await {
            assert_eq!(r.unwrap(), Ok(None));
        }
        assert_eq!(svc.shutdown().await.len(), 100);
    }

    #[tokio::test]
    async fn slow_actor_times_out_instead_of_hanging() {
        let svc = spawn_kv(16, Duration::from_millis(50));
        let kv = svc.handle();
        kv.stall(Duration::from_millis(300)).await;
        assert_eq!(kv.get("x").await, Err(KvError::Timeout));
        tokio::time::sleep(Duration::from_millis(300)).await;
        assert_eq!(kv.get("x").await, Ok(None)); // recovered
        svc.shutdown().await;
    }

    #[tokio::test]
    async fn dropping_every_handle_stops_the_actor() {
        let (tx, rx) = mpsc::channel(4);
        let (_keep, shutdown_rx) = oneshot::channel::<()>();
        let task = tokio::spawn(run_actor(rx, shutdown_rx));
        drop(tx);
        let state = tokio::time::timeout(Duration::from_secs(2), task).await.expect("actor did not stop").unwrap();
        assert!(state.is_empty());
    }

    #[test]
    #[ignore = "learner must add a test"]
    fn learner_adversarial_test() {
        todo!("fill the mailbox (capacity 1) with stalls and assert set() returns Timeout rather than blocking forever")
    }
}
