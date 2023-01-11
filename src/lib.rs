//! Dacom (Date Compiler) - Utility for searching for dates in files and analyzing the results
//!
//! Library crate root.  All the functionality
//! implemented in the binary crate is contained
//! in this crate.

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

