use std::any::Any;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::{LazyLock, Mutex};
use erased_serde::Serialize as ErasedSerialize;
use erased_serde::Deserializer;

use godot::classes::file_access::ModeFlags;
use godot::classes::{FileAccess};

pub struct Storage {
    pub items: HashMap<String, Box<dyn erased_serde::Serialize + std::marker::Send>>
}

pub static SINGLETON: LazyLock<Mutex<Storage>> = LazyLock::new(|| Mutex::new(Storage {
    items: HashMap::new()
}));

impl Storage {
    pub fn save() {
        let mut file = FileAccess::open("user://game_data.save", ModeFlags::WRITE).expect("Failed to open data file");
        let singleton = SINGLETON.lock().expect("Failed to lock singleton");
        let mut vec = Vec::new();
        {
            let json_serializer = &mut serde_json::Serializer::new(&mut vec);
            erased_serde::serialize(&singleton.items, json_serializer).expect("Failed to serialize data");
        }
        let json_string = String::from_utf8(vec).expect("Provided data did not serialize to utf8");
        file.store_string(&json_string);
    }

    pub fn load() {
        if let Some(file) = FileAccess::open("user://game_data.save", ModeFlags::READ) {
            let json = file.get_as_text();
            let string_data: &str = json.to_string().as_str();
            let deserializer = &mut serde_json::Deserializer::from_slice(string_data.as_bytes());
            let mut erased_serializer = <dyn Deserializer>::erase(deserializer);
            let data: HashMap<String, Box<dyn erased_serde::Serialize + std::marker::Send>> = erased_serde::deserialize(&mut erased_serializer).expect("Failed to deserialize");
            // HashMap<String, Box<dyn ErasedSerialize + std::marker::Send>>
            // *SINGLETON.lock().expect("Failed to lock singleton").deref_mut() = data;
        }
    }
}
