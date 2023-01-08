//! Crate library root.

// Public interfaces
pub mod date;

// Re-exports
pub use date::Month;
pub use date::Date;

// Unit tests
#[cfg(test)]
mod tests;

