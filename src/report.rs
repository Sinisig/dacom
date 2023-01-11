//! Utilities for generating a statistical report of data.

/////////////////////////////////
// Struct and enum definitions //
/////////////////////////////////

/// Struct for containing the results
/// of statistically analyzing a collection
/// of files dates.
pub struct FileAggregateReport<'l> {
   raw_data : &'l crate::collect::FileAggregateDateList,
   oldest   : &'l crate::collect::FileDateList,
   newest   : &'l crate::collect::FileDateList,
}

///////////////////////////////////
// Methods - FileAggregateReport //
///////////////////////////////////

impl<'l> FileAggregateReport<'l> {
   /// Creates a new statistical report
   /// from an existing file aggregate date
   /// set.
   pub fn from(
      file_data   : &'l crate::collect::FileAggregateDateList,
   ) -> Self {
      let oldest = &file_data.as_slice().first().unwrap();
      let newest = &file_data.as_slice().last().unwrap();

      return Self{
         raw_data : file_data,
         oldest   : oldest,
         newest   : newest,
      };
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
      write!(stream, "   {}\n", self.oldest.path().to_str().unwrap())?;
      for date in self.oldest.dates().iter() {
         write!(stream, "   {date}\n")?;
      }

      write!(stream, "\nNewest file:\n")?;
      write!(stream, "   {}\n", self.newest.path().to_str().unwrap())?;
      for date in self.newest.dates().iter() {
         write!(stream, "   {date}\n")?;
      }

      write!(stream, "\n----------- Raw Data ------------\n\n")?;

      for file in self.raw_data.iter() {
         write!(stream, "{}\n", file.path().to_str().unwrap())?;
         for date in file.dates().iter() {
            write!(stream, "   {date}\n")?;
         }
         write!(stream, "\n")?;
      }

      return Ok(());
   }
}

