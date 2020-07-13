use log::debug;

use std::collections::HashSet;
use std::sync::Arc;
use std::sync::RwLock;

pub struct StringPool {
    map: RwLock<HashSet<Arc<str>>>,
}

impl StringPool {
    pub fn new() -> Self {
        StringPool {
            map: RwLock::new(HashSet::new()),
        }
    }

    pub fn get(&self, value: &str) -> Arc<str> {
        {
            debug!("Acquiring read lock...");
            let reader = &self.map.read().unwrap();

            debug!("Checking for presence in collection...");
            if reader.contains(value) {
                debug!("Value is present in collection, cloning Arc...");
                return reader.get(value).unwrap().clone();
            }
        }

        debug!("Value not present in collection, inserting and returning Arc...");
        let mut writer = self.map.write().unwrap();
        let result: Arc<str> = Arc::from(value);
        writer.insert(result.clone());
        result
    }
}
