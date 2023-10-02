use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
pub struct Cache {
    data: Arc<RwLock<HashMap<String, String>>>,
    expiration: Arc<RwLock<HashMap<String, Instant>>>,
}

impl Cache {
    pub fn new() -> Self {
        let data = Arc::new(RwLock::new(HashMap::new()));
        let expiration = Arc::new(RwLock::new(HashMap::new()));
        let cache = Cache { data, expiration };

        let data_clone = cache.data.clone();
        let expiration_clone = cache.expiration.clone();

        thread::spawn(move || loop {
            let now = Instant::now();
            let expire: Vec<String> = {
                let ex = expiration_clone.read().unwrap();
                ex.iter()
                    .filter_map(|(key, &time)| {
                        if time <= now {
                            return Some(key.clone());
                        } else {
                            return None;
                        }
                    })
                    .collect()
            };

            if !expire.is_empty() {
                let mut data = data_clone.write().unwrap();

                for x in expire {
                    data.remove(&x);
                }
                thread::sleep(Duration::from_secs(1));
            }
        });

        cache
    }
}
