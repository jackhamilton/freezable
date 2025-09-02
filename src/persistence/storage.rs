use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use godot::classes::file_access::ModeFlags;
use godot::classes::{FileAccess};

#[typetag::serde(tag = "type")]
pub trait Freezable: Send {
    fn serialize(&self) -> &'static str;
    fn deserialize(from: &str) -> Self where Self: Sized;
}

#[derive(Deserialize)]
pub struct Storage {
    pub items: HashMap<String, Box<dyn Freezable>>
}

pub static SINGLETON: LazyLock<Mutex<Storage>> = LazyLock::new(|| Mutex::new(Storage {
    items: HashMap::new()
}));

impl Storage {
    pub fn save() {
        let mut file = FileAccess::open("user://game_data.save", ModeFlags::WRITE).expect("Failed to open data file");
        let singleton = SINGLETON.lock().expect("Failed to lock singleton");
        let mut serializable_map: HashMap<String, String> = HashMap::new();
        for key in singleton.items.keys() {
            serializable_map.insert(key.to_string(), singleton.items[key].serialize().to_string());
        }
        let json = serde_json::to_string(&serializable_map).expect("Failed to serialize");
        file.store_string(&json);
    }

    pub fn load() {
        if let Some(file) = FileAccess::open("user://game_data.save", ModeFlags::READ) {
            let json = file.get_as_text();
            let string_data = json.to_string().as_str().to_string();
            let deserialized: HashMap<String, String> = serde_json::from_str(&string_data).expect("Error deserializing");
            let mut deserialized_map: HashMap<String, Box<dyn Freezable>> = HashMap::new();
            for key in singleton.items.keys() {
                serializable_map.insert(key.to_string(), singleton.items[key].serialize().to_string());
            }
            *SINGLETON.lock().expect("Could not lock singleton") = deserialized;
        }
    }
}
