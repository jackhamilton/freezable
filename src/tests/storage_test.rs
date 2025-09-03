use serde::{Deserialize, Serialize};

use macros::freezable;
use crate::persistence::storage::{Freezable, Storage};

#[derive(Debug, PartialEq)]
#[freezable]
struct Example {
    pub field1: String,
    pub field2: i8,
    pub field3: bool,
    pub field4: f32,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            field1: "def_test".to_string(),
            field2: 7,
            field3: true,
            field4: 12.5
        }
    }
}

#[derive(Debug, PartialEq)]
#[freezable]
struct Example2 {
    pub field1: String,
    pub field2: i8,
    pub field3: bool,
    pub field4: f32,
    pub field5: f32,
}

impl Default for Example2 {
    fn default() -> Self {
        Self {
            field1: "def_test_2".to_string(),
            field2: 4,
            field3: false,
            field4: 1.0,
            field5: 0.5
        }
    }
}

#[test]
fn test_string_serialize() {
    let str = "Test";
    Storage::save("test_str", &"Test".to_string());
    let out: String = Storage::load("test_str");
    assert_eq!(out, str);
}

#[test]
fn test_struct_serialize() {
    let str = "Test";
    Storage::save("test_str", &"Test".to_string());
    let out: String = Storage::load("test_str");
    assert_eq!(out, str);
}

#[test]
fn test_freezable_serialize() {
    let test_struct = Example {
        field1: "Test".to_string(),
        field2: 4,
        field3: true,
        field4: 3.0
    };
    Storage::save("test_str", &test_struct);
    let out: Example = Storage::load("test_str");
    assert_eq!(out, test_struct);
}

#[test]
fn test_freezable_serialize_defaults() {
    let test_struct = Example::default();
    let loaded_default = Storage::load::<Example>("test_str");
    assert_eq!(loaded_default, test_struct);
}

#[test]
fn test_partial_deserialization() {
    let test_struct = Example::default();
    Storage::save("test_str", &test_struct);
    let loaded_default = Storage::load::<Example2>("test_str");
    println!("loaded: {:?}", loaded_default);
    assert_eq!(loaded_default.field1, test_struct.field1);
    assert_eq!(loaded_default.field2, test_struct.field2);
    assert_eq!(loaded_default.field3, test_struct.field3);
    assert_eq!(loaded_default.field4, test_struct.field4);
    assert_eq!(loaded_default.field5, Example2::default().field5);
}
