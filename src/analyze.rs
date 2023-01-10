//! Tools for performing a statistical anaysis of data.

/////////////////////////////////
// Struct and enum definitions //
/////////////////////////////////

/// Contains the results from analyzing the data
/// collected from FileDateAggregate.
pub struct FileDateReport<'l> {
   data  : &'l crate::FileDateAggregate,
   min   : crate::FileDate<'l>,
   max   : crate::FileDate<'l>,
}

//////////////////////////////
// Methods - FileDateReport //
//////////////////////////////

impl<'l> FileDateReport<'l> {
   /// Creates a new data report from a list
   /// of information.
   pub fn new(
      data  : &'l crate::FileDateAggregate,
      min   : crate::FileDate<'l>,
      max   : crate::FileDate<'l>,
   ) -> Self {
      return Self{
         data  : data,
         min   : min,
         max   : max,
      };
   }
}

////////////////////////////////////////////
// Trait implementations - FileDateReport //
////////////////////////////////////////////

impl<'l> std::fmt::Display for FileDateReport<'l> {
   fn fmt(
      & self,
      stream : & mut std::fmt::Formatter<'_>,
   ) -> std::fmt::Result {
      write!(stream, "---------- Data summary ----------\n\n")?;

      write!(stream, "Oldest file:\n")?;
      write!(stream, "   {}\n", self.min.path())?;
      for date in self.min.dates().as_ref() {
         write!(stream, "   {date}\n")?;
      }

      write!(stream, "\nNewest file:\n")?;
      write!(stream, "   {}\n", self.max.path())?;
      for date in self.max.dates().as_ref() {
         write!(stream, "   {date}\n")?;
      }

      write!(stream, "\n---------- Raw data ----------\n\n")?;

      for file in self.data.iter() {
         write!(stream, "{}\n", file.path())?;
         for date in file.dates().as_ref() {
            write!(stream, "   {date}\n")?;
         }
         write!(stream, "\n")?;
      }

      return Ok(());
   }
}

