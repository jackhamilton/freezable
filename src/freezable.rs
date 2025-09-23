use serde::Deserialize;
use serde::Serialize;

pub trait Freezable: Serialize + for<'a> Deserialize<'a> {}

impl<T: Serialize + for<'a> Deserialize<'a>> Freezable for T {}
