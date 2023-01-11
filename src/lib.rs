//! Crate library root.

// Public interfaces
pub mod args;
pub mod collect;
pub mod date;
pub mod report;

// Re-exports
pub use args::Args;
pub use collect::CollectDateError;
pub use collect::DateList;
pub use collect::FileDateList;
pub use collect::FileAggregateDateList;
pub use date::Month;
pub use date::Date;
pub use report::FileAggregateReport;

// Unit tests
#[cfg(test)]
mod tests;

