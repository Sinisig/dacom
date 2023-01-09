//! Utilities for aggregating and analyzing data.


/* Struct and enum definitions */


/// A sorted list of dates from oldest to
/// newest.
pub struct DateList<'l> {
   list  : &'l [crate::Date],
}

/// An enum containing an error involving
/// file dating.
#[derive(Copy, Clone, Debug)]
pub enum FileDateError {
   IOError,
}

/// A single file date collection element created
/// by FileDateAggregateIterator.
pub struct FileDate<'l> {
   file_name   : &'l str,
   date_list   : &'l DateList<'l>,
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


/* Methods - DateList */


impl<'l> DateList<'l> {
   /// Creates a new Date List from an existing
   /// sorted array.
   pub fn from(
      date_list   : &'l [crate::Date],
   ) -> Self {
      return Self{
         list  : date_list,
      }
   }

   /// Gets a reference to the sorted Date array.
   /// This is implemented in the Deref trait, so
   /// it's usually unecessary to call this method
   /// explicitly as opposed to dereferencing the
   /// struct directly.
   pub fn as_ref(
      &'l self,
   ) -> &'l [crate::Date] {
      return self.list;
   }
}


/* Trait implementations - DateList */


impl<'l> std::ops::Deref for DateList<'l> {
   type Target = [crate::Date];

   fn deref(&self) -> &Self::Target {
      return &self.list;
   }
}


/* Methods - FileDate */


impl<'l> FileDate<'l> {
   /// Creates a new File Date from a
   /// file path and a DateList.
   pub fn from(
      file_name   : &'l str,
      date_list   : &'l DateList<'l>,
   ) -> Self {
      return Self{
         file_name   : file_name,
         date_list   : date_list,
      };
   }

   /// Gets a reference to the file path.
   pub fn path(
      &'l self,
   ) -> &'l str {
      return self.file_name;
   }

   /// Gets a reference to the date list.
   pub fn dates(
      &'l self,
   ) -> &'l DateList {
      return &self.date_list;
   }
}


/* Methods - FileDateAggregate */


impl<'l> FileDateAggregate<'l> {
   // Searches a file for files with dating
   // information and stores those with found
   // dates.  Directories are recursively searched.
   // The input closure is executed for every
   // file searched.
   pub fn new<F>(
      path     : &'l str,
      per_file : F,
   ) -> Result<Self, FileDateError>
   where F: Fn(&'l str) {
      // TODO: Implement
      per_file(path);
      return Err(FileDateError::IOError);
   }

   /// Creates an iterator over the elements.
   pub fn iter(
      &'l self,
   ) -> FileDateAggregateIterator<'l> {
      return FileDateAggregateIterator::new(self);
   }

   /// Gets the file count.
   pub fn count(
      &'l self,
   ) -> usize {
      return self.file_names.len();
   }
}


/* Methods - FileDateAggregateIterator */


impl<'l> FileDateAggregateIterator<'l> {
   /// Creates a new iterator from an existing
   /// FileDateAggregate.
   pub fn new(
      file_data_aggregate  : &'l FileDateAggregate
   ) -> Self {
      return Self{
         index : 0,
         data  : file_data_aggregate,
      };
   }
}


/* Trait implementations - FileDateAggregateIterator */


impl<'l> std::iter::Iterator for FileDateAggregateIterator<'l> {
   type Item = FileDate<'l>;

   fn next(& mut self) -> Option<Self::Item> {
      if self.index >= self.data.count() {
         return None;
      }

      return Some(FileDate::from(
         &self.data.file_names[self.index],
         &self.data.file_dates[self.index],
      ));
   }
}


/* Trait implementations - FileDateError */


impl std::fmt::Display for FileDateError {
   fn fmt(
      & self,
      stream   : & mut std::fmt::Formatter<'_>,
   ) -> std::fmt::Result {
      return write!(stream, "{}", match self {
         Self::IOError  => "I/O Error",
      });
   }
}

impl std::error::Error for FileDateError {
}

