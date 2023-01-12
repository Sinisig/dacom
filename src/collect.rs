//! Utilities for aggregating and analyzing data.
//!
//! The idea is to store a sorted list of dates
//! along with a file path inside a sorted array
//! and collect the results into a struct.
//! A FileAggregateDateList is the struct which
//! stores all these results, and its creation
//! methods should be used to create file dating
//! information.  The file aggregate struct is
//! a list of FileDateList elements, which contain
//! a file path and a list of dates.  These dates
//! are stored in a DateList struct, which stores
//! a sorted list of dates.

/////////////////////////////////
// Struct and enum definitions //
/////////////////////////////////

/// An error type detailing the reason behind
/// a function erroring out.
#[derive(Copy, Clone, Debug)]
pub enum CollectDateError {
   /// Inadequate permission opening
   /// a file or directory.
   PermissionDenied,

   /// A file or directory does not
   /// exist at the given path.
   FileNotFound,

   /// A general I/O error occurred.
   GeneralIOError,

   /// A directory path was passed
   /// to a function expecting a file
   /// path.
   FileIsDirectory,

   /// Input data contains invalid
   /// data.  This usually results
   /// when a file parsing function
   /// which expects text receives
   /// a file containing binary data.
   InvalidData,
}

/// A type alias for a standard result
/// type with CollectDateError as the
/// error type.
pub type Result<T> = std::result::Result<T, CollectDateError>;

/// A list of dates sorted from oldest
/// to newest.
pub struct DateList {
   list  : sorted_vec::SortedVec<crate::date::Date>,
}

/// An iterator over a DateList created
/// with the iter() method.
pub struct DateListIterator<'l> {
   date_set : &'l DateList,
   index    : usize,
}

/// A DateList gathered from a file on disk,
/// storing the path to the file.
pub struct FileDateList {
   path  : std::path::PathBuf,
   dates : DateList,
}

/// A sorted list of many different files
/// containing their collected dates.
pub struct FileAggregateDateList {
   files : sorted_vec::SortedVec<FileDateList>,
}

/// An iterator over a FileAggregateDateList created
/// with the iter() method.
pub struct FileAggregateDateListIterator<'l> {
   data  : &'l FileAggregateDateList,
   index : usize,
}

//////////////////////////////////////////////
// Trait implementations - CollectDateError //
//////////////////////////////////////////////

impl std::error::Error for CollectDateError {
}

impl std::fmt::Display for CollectDateError {
   fn fmt(
      & self,
      stream : & mut std::fmt::Formatter,
   ) -> std::fmt::Result {
      return write!(stream, "{}", match self {
         Self::PermissionDenied
            => "Permission denied",
         Self::FileNotFound
            => "File does not exist",
         Self::GeneralIOError
            => "General I/O error",
         Self::FileIsDirectory
            => "File is a directory",
         Self::InvalidData
            => "Invalid data",
      });
   }
}

impl std::convert::From<std::io::Error> for CollectDateError {
   fn from(
      value : std::io::Error,
   ) -> Self {
      use std::io::ErrorKind::*;

      let value = value.kind();
      match value {
         NotFound
            => Self::FileNotFound,
         PermissionDenied
            => Self::PermissionDenied,
         InvalidData
            => Self::InvalidData,
         _
            => Self::GeneralIOError,
      }
   }
}

////////////////////////
// Methods - DateList //
////////////////////////

impl DateList {
   /// Creates a new DateList from
   /// an existing sorted list.
   pub fn from(
      list : sorted_vec::SortedVec<crate::date::Date>,
   ) -> Self {
      return Self{
         list  : list,
      };
   }

   /// Gets a reference to the underlying
   /// data slice.
   pub fn as_slice<'l>(
      &'l self,
   ) -> &'l [crate::date::Date] {
      return &self.list;
   }

   /// Creates an iterator over the
   /// dates stored in the date set.
   pub fn iter<'l>(
      &'l self,
   ) -> DateListIterator<'l> {
      return DateListIterator::new(self);
   }
}

//////////////////////////////////////
// Trait implementations - DateList //
//////////////////////////////////////

impl std::ops::Deref for DateList {
   type Target = [crate::date::Date];

   fn deref(
      & self,
   ) -> & Self::Target {
      return self.as_slice();
   }
}

impl std::cmp::PartialEq for DateList {
   fn eq(
      & self,
      other : & Self,
   ) -> bool {
      return self.as_slice().eq(other.as_slice());
   }
}

impl std::cmp::PartialOrd for DateList {
   fn partial_cmp(
      & self,
      other : & Self,
   ) -> Option<std::cmp::Ordering> {
      use std::cmp::Ordering::*;

      // Ordering works by looking first at the
      // oldest date and comparing.  If they are
      // equal, they compare the newest date.
      // If they are still equal, compare the
      // element count.
      return match self.first()?.partial_cmp(other.first()?)? {
         Greater  => Some(Greater),
         Less     => Some(Less),
         Equal    => self.last()?.partial_cmp(other.last()?),
      };
   }
}

impl std::cmp::Eq for DateList {
}

impl std::cmp::Ord for DateList {
   fn cmp(
      & self,
      other : & Self,
   ) -> std::cmp::Ordering {
      use std::cmp::Ordering::*;

      return self.partial_cmp(other).unwrap_or_else(|| Equal);
   }
}

////////////////////////////////
// Methods - DateListIterator //
////////////////////////////////

impl<'l> DateListIterator<'l> {
   /// Creates a new DateListIterator which
   /// operates on the given DateList reference.
   pub fn new(
      date_set : &'l DateList,
   ) -> Self {
      return Self{
         date_set : date_set,
         index    : 0,
      };
   }
}

//////////////////////////////////////////////
// Trait implementations - DateListIterator //
//////////////////////////////////////////////

impl<'l> std::iter::Iterator for DateListIterator<'l> {
   type Item = &'l crate::date::Date;

   fn next(
      & mut self,
   ) -> Option<Self::Item> {
      let item = self.date_set.as_slice().get(self.index);

      self.index += 1;
      return item;
   }
}

////////////////////////////
// Methods - FileDateList //
////////////////////////////

impl FileDateList {
   /// Creates a new FileDateList from
   /// an existing path buffer and
   /// date set.
   pub fn from(
      path  : std::path::PathBuf,
      dates : DateList,
   ) -> Self {
      return Self{
         path  : path,
         dates : dates,
      };
   }
   
   /// Get a reference to the file's
   /// path.
   pub fn path<'l>(
      &'l self,
   ) -> &'l std::path::Path {
      return &self.path;
   }

   /// Get a reference to the file's
   /// collected date set.
   pub fn dates<'l>(
      &'l self,
   ) -> &'l DateList {
      return &self.dates;
   }
}

//////////////////////////////////////////
// Trait implementations - FileDateList //
//////////////////////////////////////////

impl std::cmp::PartialEq for FileDateList {
   fn eq(
      & self,
      other : & Self,
   ) -> bool {
      return self.path().eq(other.path()) && self.dates().eq(other.dates());
   }
}

impl std::cmp::PartialOrd for FileDateList {
   fn partial_cmp(
      & self,
      other : & Self,
   ) -> Option<std::cmp::Ordering> {
      use std::cmp::Ordering::*;

      // Date order takes precedence over file
      // name.
      return match self.dates().cmp(other.dates()) {
         Greater  => Some(Greater),
         Less     => Some(Less),
         Equal    => self.path().partial_cmp(other.path()),
      };
   }
}

impl std::cmp::Eq for FileDateList {
}

impl std::cmp::Ord for FileDateList {
   fn cmp(
      & self,
      other : & Self,
   ) -> std::cmp::Ordering {
      use std::cmp::Ordering::*;

      return self.partial_cmp(other).unwrap_or_else(|| Equal);
   }
}

//////////////////////////////////////////////
// Internal helpers - FileAggregateDateList //
//////////////////////////////////////////////

impl FileAggregateDateList {
   /// Searches an input text stream for
   /// dates and return a DateList containing
   /// the found dates.
   fn internal_collect_dates(
      text  : & str,
   ) -> DateList {
      // Get an unsorted list of dates
      let dates = crate::date::Date::from_text_multi_sorted(text);

      // Sort and remove duplicates
      let dates = DateList::from(dates);

      // Return success
      return dates;
   }

   /// Searches a single file for dates and returns
   /// a DateList containing all the found dates.
   fn internal_search_file_single<F>(
      path     : & std::path::Path,
      per_file : F,
   ) -> Result<DateList>
   where F: Fn(& std::path::Path) {
      // Execute the user closure
      per_file(path);

      // Check if the file is a directory
      if match std::fs::metadata(path) {
         Ok(md)   => md,
         Err(e)   => return Err(e.into()),
      }.is_dir() == true {
         return Err(CollectDateError::FileIsDirectory);
      }

      // Map the file into memory as a string slice
      let file = match std::fs::File::open(path) {
         Ok(f)    => f,
         Err(e)   => return Err(e.into()),
      };
      let file = match unsafe{memmap2::Mmap::map(&file)} {
         Ok(m)    => m,
         Err(e)   => return Err(e.into()),
      };
      let file = match std::str::from_utf8(&file) {
         Ok(d)    => d,
         Err(_)   => return Err(CollectDateError::InvalidData),
      };

      // Search for dates within the file
      let list = Self::internal_collect_dates(&file);

      // Return success
      return Ok(list);
   }

   /// Searches a file path for dates in a file,
   /// searching all files recursively in any
   /// directories encountered.  The file data is
   /// not sorted in this function.
   fn internal_search_dir_recursive_unsorted<F>(
      file_list_buffer  : & mut sorted_vec::SortedVec<FileDateList>,
      path              : std::path::PathBuf,
      per_file          : F,
   ) -> Result<()>
   where F: Fn(& std::path::Path) + Copy {
      // Try to parse the file path, if it errors as a directory, recursively
      // search that directory
      match Self::internal_search_file_single(&path, per_file) {
         Ok(date_set)  => {
            // If the data list contains dates, sort the dates
            // and add them and the path to the buffer
            if date_set.is_empty() == false {
               file_list_buffer.push(FileDateList::from(
                  path,
                  date_set,
               ));
            }
         },
         Err(err)       => {
            // If it's a directory, do nothing
            // If it's binary data, exit early
            // Otherwise return the error
            match err {
               CollectDateError::FileIsDirectory
                  => (),
               CollectDateError::InvalidData
                  => return Ok(()),
               _
                  => return Err(err),
            }

            // Iterate for every file in the directory and try to parse it
            for file in match std::fs::read_dir(&path) {
               Ok(iter) => iter,
               Err(err) => return Err(err.into()),
            } {
               // Unwrap the file result, erroring if we get a
               // file I/O error
               let file = match file {
                  Ok(f)    => f,
                  Err(e)   => return Err(e.into()),
               };

               // Try to parse the new file/directory
               Self::internal_search_dir_recursive_unsorted(
                  file_list_buffer,
                  file.path(),
                  per_file,
               )?;
            }
         },
      }

      // Return success
      return Ok(());
   }
}

/////////////////////////////////////
// Methods - FileAggregateDateList //
/////////////////////////////////////

impl FileAggregateDateList {
   /// Recursively searches a file or directory
   /// for dating information and stores them in
   /// a sorted set with no duplicates and no
   /// files with zero found dates.
   pub fn new_recursive<P>(
      path  : P,
   ) -> Result<Self>
   where P: AsRef<std::path::Path> {
      // Closure does nothing
      return Self::new_recursive_with(
         path,
         |_| {},
      );
   }

   /// Executes the same as Self::new_recursive,
   /// but a user closure is executed for each file
   /// that is searched.  The closure is passed the
   /// path to the current file.
   pub fn new_recursive_with<P, F>(
      path     : P,
      per_file : F,
   ) -> Result<Self>
   where P: AsRef<std::path::Path>,
         F: Fn(& std::path::Path) + Copy {
      // Convert the path into a PathBuf
      let mut path_buf = std::path::PathBuf::new();
      path_buf.push(path);

      // Create the buffer for holding file date lists
      let mut file_list_buffer = sorted_vec::SortedVec::new();

      // Populate the buffer with file into
      Self::internal_search_dir_recursive_unsorted(
         & mut file_list_buffer,
         path_buf,
         per_file,
      )?;

      // Create the struct
      let aggregate = Self{
         files : file_list_buffer,
      };

      // Return success
      return Ok(aggregate);
   }

   /// Accesses the underlying data as
   /// a FileDateList slice.  This is
   /// equivalent to the Deref trait
   /// which is implemented.
   pub fn as_slice<'l>(
      &'l self,
   ) -> &'l [FileDateList] {
      return &self.files;
   }

   /// Creates an iterator over the file
   /// date sets.
   pub fn iter<'l>(
      &'l self,
   ) -> FileAggregateDateListIterator<'l> {
      return FileAggregateDateListIterator::new(self);
   }

   /// Creates a FileAggregateReport from
   /// the data.
   pub fn create_report<'l>(
      &'l self,
   ) -> crate::report::Result<crate::report::FileAggregateReport<'l>> {
      return crate::report::FileAggregateReport::from(self);
   }
}

///////////////////////////////////////////////////
// Trait implementations - FileAggregateDateList //
///////////////////////////////////////////////////

impl std::ops::Deref for FileAggregateDateList {
   type Target = [FileDateList];

   fn deref(
      & self,
   ) -> & Self::Target {
      return self.as_slice();
   }
}

/////////////////////////////////////////////
// Methods - FileAggregateDateListIterator //
/////////////////////////////////////////////

impl<'l> FileAggregateDateListIterator<'l> {
   /// Creates a new iterator from an
   /// existing FileAggregateDateList.
   pub fn new(
      data : &'l FileAggregateDateList,
   ) -> Self {
      return Self{
         data  : data,
         index : 0,
      };
   }
}

///////////////////////////////////////////////////////////
// Trait implementations - FileAggregateDateListIterator //
///////////////////////////////////////////////////////////

impl<'l> std::iter::Iterator for FileAggregateDateListIterator<'l> {
   type Item = &'l FileDateList;

   fn next(
      & mut self,
   ) -> Option<Self::Item> {
      let item = self.data.as_slice().get(self.index);

      self.index += 1;
      return item;
   }
}

