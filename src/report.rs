//! Utilities for generating a statistical report of data.

/////////////////////////////////
// Struct and enum definitions //
/////////////////////////////////

/// An error type representing an error
/// relating to a data report.
#[derive(Copy, Clone, Debug)]
pub enum ReportError {
   NoData,
   InvalidData,
}

/// A type alias for the standard library
/// result type with an error variant of
/// ReportError.
pub type Result<T> = std::result::Result<T, ReportError>;

/// Struct for containing the results
/// of statistically analyzing a collection
/// of files dates.
pub struct FileAggregateReport<'l> {
   raw_data : &'l crate::collect::FileAggregateDateList,
   oldest   : &'l crate::collect::FileDateList,
   newest   : &'l crate::collect::FileDateList,
   median   : &'l crate::collect::FileDateList,
}

/////////////////////////////////////////
// Trait implementations - ReportError //
/////////////////////////////////////////

impl std::error::Error for ReportError {
}

impl std::fmt::Display for ReportError {
   fn fmt(
      & self,
      stream : & mut std::fmt::Formatter<'_>,
   ) -> std::fmt::Result {
      return write!(stream, "{}", match self {
         Self::NoData
            => "No input data",
         Self::InvalidData
            => "Invalid input data",
      });
   }
}

///////////////////////////////////
// Methods - FileAggregateReport //
///////////////////////////////////

impl<'l> FileAggregateReport<'l> {
   /// Creates a new statistical report
   /// from an existing file aggregate date
   /// set.  If a report cannot be formed
   /// from the data, an error is returned.
   pub fn from(
      file_data   : &'l crate::collect::FileAggregateDateList,
   ) -> Result<Self> {
      // Get statistical variables
      let oldest = match file_data.first() {
         Some(fd) => fd,
         None     => return Err(ReportError::NoData),
      };
      let newest = match file_data.last() {
         Some(fd) => fd,
         None     => return Err(ReportError::NoData),
      };
      let median = match file_data.get(file_data.len() / 2) {
         Some(fd) => fd,
         None     => return Err(ReportError::NoData),
      };

      // Create struct instance
      let report = Self{
         raw_data : file_data,
         oldest   : oldest,
         newest   : newest,
         median   : median,
      };

      // Return success
      return Ok(report);
   }
}

/////////////////////////////////////////////////
// Trait implementations - FileAggregateReport //
/////////////////////////////////////////////////

impl<'l> std::fmt::Display for FileAggregateReport<'l> {
   fn fmt(
      & self,
      stream : & mut std::fmt::Formatter<'_>,
   ) -> std::fmt::Result {
      write!(stream, "--------- Data Summary ----------\n\n")?;

      write!(stream, "Oldest file:\n")?;
      write!(stream, "   {}\n", self.oldest.path().to_str().unwrap_or("???"))?;
      for date in self.oldest.dates().iter() {
         write!(stream, "   {date}\n")?;
      }

      write!(stream, "\nNewest file:\n")?;
      write!(stream, "   {}\n", self.newest.path().to_str().unwrap_or("???"))?;
      for date in self.newest.dates().iter() {
         write!(stream, "   {date}\n")?;
      }

      write!(stream, "\nMedian file:\n")?;
      write!(stream, "   {}\n", self.median.path().to_str().unwrap_or("???"))?;
      for date in self.median.dates().iter() {
         write!(stream, "   {date}\n")?;
      }

      write!(stream, "\n----------- Raw Data ------------\n\n")?;

      for file in self.raw_data.iter() {
         write!(stream, "{}\n", file.path().to_str().unwrap_or("???"))?;
         for date in file.dates().iter() {
            write!(stream, "   {date}\n")?;
         }
         write!(stream, "\n")?;
      }

      return Ok(());
   }
}

