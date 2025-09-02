use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

pub static SINGLETON: LazyLock<Mutex<Storage>> = LazyLock::new(|| Mutex::new(Storage {
    items: HashMap::new()
}));

pub trait Freezable: Serialize + for<'a> Deserialize<'a> {}

impl Freezable for String {}

pub struct Storage {
    items: HashMap<String, String>
}

impl Storage {
    pub fn save<T: Freezable>(key: String, item: T) {
        let string = serde_json::to_string(&item).expect("Error serializing string");
        SINGLETON.lock().expect("Could not lock storage").items.insert(key, string);
    }
}
