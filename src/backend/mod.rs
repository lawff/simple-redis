use std::{ops::Deref, sync::Arc};

use dashmap::{DashMap, DashSet};

use crate::RespFrame;

#[derive(Clone)]
pub struct Backend(Arc<BackendInner>);

pub struct BackendInner {
    map: DashMap<String, RespFrame>,
    hmap: DashMap<String, DashMap<String, RespFrame>>,
    // RespFrame 没有eq和hash，所以不能直接用RespFrame作为set的value
    set: DashMap<String, DashSet<String>>,
}

impl Deref for Backend {
    type Target = BackendInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self(Arc::new(BackendInner::default()))
    }
}

impl Default for BackendInner {
    fn default() -> Self {
        Self {
            map: DashMap::new(),
            hmap: DashMap::new(),
            set: DashMap::new(),
        }
    }
}

impl Backend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<RespFrame> {
        self.map.get(key).map(|v| v.value().clone())
    }

    pub fn set(&self, key: String, value: RespFrame) {
        self.map.insert(key, value);
    }

    pub fn hget(&self, key: &str, field: &str) -> Option<RespFrame> {
        self.hmap
            .get(key)
            .and_then(|m| m.get(field).map(|v| v.value().clone()))
    }

    pub fn hset(&self, key: String, field: String, value: RespFrame) {
        let hmap = self.hmap.entry(key).or_default();
        hmap.insert(field, value);
    }

    pub fn hgetall(&self, key: &str) -> Option<DashMap<String, RespFrame>> {
        self.hmap.get(key).map(|m| m.clone())
    }

    pub fn sadd(&self, key: String, value: Vec<String>) -> i64 {
        let set = self.set.entry(key).or_default();

        let mut inserted_count = 0;
        for v in value {
            if set.insert(v) {
                inserted_count += 1;
            }
        }
        inserted_count
    }

    pub fn smembers(&self, key: &str) -> Option<DashSet<String>> {
        self.set.get(key).map(|s| s.clone())
    }
}
