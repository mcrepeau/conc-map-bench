use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use bustle::*;

use super::Value;

#[derive(Clone)]
pub struct PapayaTable<K, H> {
    map: Arc<papaya::HashMap<K, Value, H>>,
}

impl<K, H> Collection for PapayaTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            map: Arc::new(papaya::HashMap::with_capacity_and_hasher(
                capacity,
                H::default(),
            )),
        }
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for PapayaTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.map.pin().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.map.pin().insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.map.pin().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let pinned = self.map.pin();
        pinned.get(key)
            .and_then(|v| pinned.insert(*key, v + 1))
            .is_some()
    }
}