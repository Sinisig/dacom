//! Crate library root.

// Public interfaces
pub mod analyze;
pub mod args;
pub mod collect;
pub mod date;

// Re-exports
pub use analyze::FileDateReport;
pub use args::Args;
pub use collect::DateList;
pub use collect::FileDateError;
pub use collect::FileDate;
pub use collect::FileDateAggregate;
pub use collect::FileDateAggregateIterator;
pub use date::Month;
pub use date::Date;

// Unit tests
#[cfg(test)]
mod tests;

