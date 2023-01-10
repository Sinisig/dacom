//! Crate library root.

// Public interfaces
pub mod args;
pub mod collect;
pub mod date;

// Re-exports
pub use args::Args;
pub use collect::CollectDateError;
pub use collect::DateSet;
pub use collect::FileDateSet;
pub use collect::FileAggregateDateSet;
pub use date::Month;
pub use date::Date;

// Unit tests
#[cfg(test)]
mod tests;

