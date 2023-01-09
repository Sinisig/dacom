//! Utilities for aggregating and analyzing data.


/* Struct and enum definitions */


/// A sorted list of dates from oldest to
/// newest.
pub struct DateList<'l> {
   list  : &'l [crate::Date],
}

/// A single file date collection element created
/// by FileDateAggregateIterator.
pub struct FileDate<'l> {
   file_name   : &'l str,
   dates       : DateList<'l>,
}

/// A collection of files with their dating
/// information.
pub struct FileDateAggregate<'l> {
   // Implemented as SoA for better cache performance.
   // Each element of file_names corresponds 1:1 with
   // an element of the same index from file_dates.
   file_names  : Vec<String>,
   file_dates  : Vec<DateList<'l>>,
}

/// An iterator over a FileDateAggregate object.
pub struct FileDateAggregateIterator<'l> {
   index : usize,
   data  : &'l FileDateAggregate<'l>,
}

