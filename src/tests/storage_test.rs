use crate as freezable_trait;
use std::fmt::Debug;
use freezable_macros::freezable;

#[derive(Debug, PartialEq, Clone)]
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

fn validate_ser<T: freezable_trait::Freezable + Debug + PartialEq>(item: T) {
    let string = serde_json::to_string(&item).expect("Error serializing string");
    let deser: T = serde_json::from_str(&string).expect("Failed to deserialize");
    assert_eq!(item, deser);
}

fn ser_deser<T: freezable_trait::Freezable + Debug + PartialEq, G: freezable_trait::Freezable>(item: T) -> G {
    let string = serde_json::to_string(&item).expect("Error serializing string");
    let deser: G = serde_json::from_str(&string).expect("Failed to deserialize");
    deser
}

#[test]
fn test_string_serialize() {
    let str = "Test";
    validate_ser(str.to_string());
}
#[test]
fn test_freezable_serialize() {
    let test_struct = Example {
        field1: "Test".to_string(),
        field2: 4,
        field3: true,
        field4: 3.0
    };
    validate_ser(test_struct);
}

#[test]
fn test_freezable_serialize_defaults() {
    let test_struct = Example::default();
    let reser: Example = ser_deser(test_struct.clone());
    assert_eq!(reser, test_struct);
}

#[test]
fn test_partial_deserialization() {
    let test_struct = Example::default();
    let resered: Example2 = ser_deser(test_struct.clone());
    assert_eq!(resered.field1, test_struct.field1);
    assert_eq!(resered.field2, test_struct.field2);
    assert_eq!(resered.field3, test_struct.field3);
    assert_eq!(resered.field4, test_struct.field4);
    assert_eq!(resered.field5, Example2::default().field5);
}
