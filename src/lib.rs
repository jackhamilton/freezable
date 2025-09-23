mod freezable;

pub use crate::freezable::Freezable;
pub use freezable_macros::freezable;

#[cfg(test)]
mod tests;
