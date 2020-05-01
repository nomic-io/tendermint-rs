// TODO: Replace this in-memory store with a proper `sled` based implementation

use std::{
    collections::BTreeMap,
    marker::PhantomData,
    sync::{Arc, RwLock},
};

use crate::prelude::*;

#[derive(Debug)]
pub struct Trusted;

#[derive(Debug)]
pub struct Untrusted;

#[derive(Debug, Default)]
pub struct Store<T> {
    store: BTreeMap<Height, LightBlock>,
    marker: PhantomData<T>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
            marker: PhantomData,
        }
    }

    pub fn split(self) -> (StoreReader<T>, StoreReadWriter<T>) {
        let store = Arc::new(RwLock::new(self));

        let reader = StoreReader {
            store: store.clone(),
        };

        let writer = StoreReadWriter { store };

        (reader, writer)
    }
}

impl<T> Store<T> {
    pub fn get(&self, height: Height) -> Option<&LightBlock> {
        self.store.get(&height)
    }

    pub fn add(&mut self, light_block: LightBlock) {
        self.store.insert(light_block.height, light_block);
    }

    pub fn all(&self) -> Vec<&LightBlock> {
        self.store.values().collect()
    }

    pub fn latest_height(&self) -> Option<Height> {
        self.store.keys().last().copied()
    }

    pub fn latest(&self) -> Option<&LightBlock> {
        self.latest_height().and_then(|h| self.get(h))
    }
}

#[derive(Clone, Debug)]
pub struct StoreReader<T> {
    store: Arc<RwLock<Store<T>>>,
}

impl<T> StoreReader<T> {
    pub fn get(&self, height: Height) -> Option<LightBlock> {
        self.store.read().unwrap().get(height).cloned()
    }

    pub fn latest_height(&self) -> Option<Height> {
        self.store.read().unwrap().latest_height()
    }

    pub fn latest(&self) -> Option<LightBlock> {
        self.store.read().unwrap().latest().cloned()
    }

    pub fn all(&self) -> Vec<LightBlock> {
        self.store
            .read()
            .unwrap()
            .all()
            .into_iter()
            .cloned()
            .collect()
    }
}

#[derive(Debug)]
pub struct StoreReadWriter<T> {
    store: Arc<RwLock<Store<T>>>,
}

impl<T> StoreReadWriter<T> {
    pub fn add(&mut self, light_block: LightBlock) {
        let mut store = self.store.write().unwrap();
        store.add(light_block);
    }
}
