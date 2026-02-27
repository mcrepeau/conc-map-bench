use std::hash::Hash;
use std::sync::Arc;

use bustle::*;
use parking_lot::RwLock;

use super::Value;

#[derive(Clone)]
pub struct PinHashTable<K>(Arc<RwLock<pinhash::HashMap<K, Value>>>);

impl<K> Collection for PinHashTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(RwLock::new(pinhash::HashMap::with_capacity(capacity))))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K> CollectionHandle for PinHashTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.read().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.read().insert(*key, 0).is_ok()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.write().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.write().get_mut(key).map(|v| *v += 1).is_some()
    }
}
