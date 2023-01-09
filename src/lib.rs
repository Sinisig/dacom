//! Crate library root.

// Public interfaces
pub mod args;
pub mod date;
pub mod stat;

// Re-exports
pub use args::Args;
pub use date::Month;
pub use date::Date;
pub use stat::DateList;
pub use stat::FileDateError;
pub use stat::FileDate;
pub use stat::FileDateAggregate;
pub use stat::FileDateAggregateIterator;

// Unit tests
#[cfg(test)]
mod tests;

