use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use bustle::*;

use super::Value;

#[derive(Clone)]
pub struct PinHashTable<K, H>(Arc<pinhash::HashMap<K, Value, H>>);

impl<K, H> Collection for PinHashTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(pinhash::HashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        )))
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
        self.0.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, 0)
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key)
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.modify(key, |v| *v += 1)
    }
}