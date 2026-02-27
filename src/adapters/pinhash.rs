use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use bustle::*;

use super::Value;

#[derive(Clone)]
pub struct PinHashTable<K, H> {
    map: Arc<pinhash::HashMap<K, Value, H>>,
}

impl<K,H> Collection for PinHashTable<K,H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            map: Arc::new(pinhash::HashMap::with_capacity_and_hasher(
                capacity,
                H::default(),
            ))
        }
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for PinHashTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.map.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.map.insert(*key, 0).is_ok()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.map.remove(key)
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.map.modify(key, |v| *v += 1)
    }
}
