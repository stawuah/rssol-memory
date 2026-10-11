//! # Sharded — many small locks instead of one big one
//!
//! **Like you're five:** one checkout lane = a long line. Sixteen lanes, and
//! you go to the lane matching the first letter of your name = short lines.
//!
//! **Use it when:** one `shared!` HashMap becomes the bottleneck (threads
//! mostly wait for the lock, not work). Accounts, sessions, peer tables,
//! caches. Each key always lands in the same shard (by hash), so two threads
//! only wait for each other if their keys share a shard.
//!
//! The cost: operations across many keys (like "total of all balances")
//! must visit every shard, and are not one atomic snapshot.

use crate::{Arc, Mutex, MutexGuard};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub struct Sharded<K, V> {
    shards: Arc<Vec<Mutex<HashMap<K, V>>>>,
}

impl<K: Hash + Eq, V> Sharded<K, V> {
    pub fn new(shard_count: usize) -> Self {
        assert!(shard_count > 0);
        Sharded { shards: Arc::new((0..shard_count).map(|_| Mutex::new(HashMap::new())).collect()) }
    }

    fn index(&self, key: &K) -> usize {
        let mut h = DefaultHasher::new();
        key.hash(&mut h);
        (h.finish() as usize) % self.shards.len()
    }

    /// Lock ONLY the shard holding `key`. Use the guard like a HashMap.
    pub fn shard(&self, key: &K) -> MutexGuard<'_, HashMap<K, V>> {
        self.shards[self.index(key)].lock()
    }

    /// Visit every shard, one at a time.
    pub fn for_each_shard(&self, mut f: impl FnMut(&mut HashMap<K, V>)) {
        for s in self.shards.iter() {
            f(&mut s.lock());
        }
    }

    pub fn len(&self) -> usize {
        self.shards.iter().map(|s| s.lock().len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<K, V> Clone for Sharded<K, V> {
    fn clone(&self) -> Self {
        Sharded { shards: self.shards.clone() }
    }
}
