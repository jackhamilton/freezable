mod freezable;

pub use crate::freezable::Freezable;
pub use freezable_macros::freezable;
pub use serde as _serde;

#[cfg(test)]
mod tests;
