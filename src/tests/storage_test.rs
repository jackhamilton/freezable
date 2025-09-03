use serde::{Deserialize, Serialize};

use crate::freezable;
use crate::persistence::storage::{Freezable, Storage};

#[freezable]
struct Example {
    pub field1: String,
    pub field2: i8,
    pub field3: bool,
    pub field4: f32,
}

#[derive(Default)]
#[derive(Serialize, Deserialize)]
struct Example2 {
    #[serde(default)]
    pub field1: String,
    #[serde(default)]
    pub field2: i8,
    #[serde(default)]
    pub field3: bool,
    #[serde(default)]
    pub field4: f32,
}

impl Freezable for Example2 {}

#[test]
fn test_string_serialize() {
    let str = "Test";
    Storage::save("test_str", "Test".to_string());
    let out: String = Storage::load("test_str");
    assert_eq!(out, str);
}

#[test]
fn test_struct_serialize() {
    let str = "Test";
    Storage::save("test_str", "Test".to_string());
    let out: String = Storage::load("test_str");
    assert_eq!(out, str);
}
