use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;

pub trait Freezable: Serialize + for<'a> Deserialize<'a> {}

impl Freezable for String {}
impl Freezable for i8 {}
impl Freezable for i16 {}
impl Freezable for i32 {}
impl Freezable for i64 {}
impl Freezable for f32 {}
impl Freezable for f64 {}
impl Freezable for u8 {}
impl Freezable for u16 {}
impl Freezable for u32 {}
impl Freezable for u64 {}
impl Freezable for bool {}

impl<T: Freezable> Freezable for Vec<T> {}
impl<T: Freezable + std::cmp::Eq + std::hash::Hash, G: Freezable> Freezable for HashMap<T, G> {}
