//! Crate library root.

// Public interfaces
pub mod args;
pub mod collect;
pub mod date;

// Re-exports
pub use args::Args;
pub use date::Month;
pub use date::Date;

// Unit tests
#[cfg(test)]
mod tests;

