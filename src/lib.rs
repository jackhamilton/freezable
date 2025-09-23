mod freezable;

pub use crate::freezable::Freezable;
pub use freezable_macros::freezable;

pub use serde::Deserialize;
pub use serde::Serialize;

#[cfg(test)]
mod tests;
