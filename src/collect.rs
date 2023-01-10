//! Utilities for aggregating and analyzing data.

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
/// to newest without duplicates.
pub struct DateSet {
   list  : sorted_vec::SortedSet<crate::date::Date>,
}

/// A DateSet gathered from a file on disk,
/// storing the path to the file.
pub struct FileDateSet {
   path  : std::path::PathBuf,
   dates : DateSet,
}

/// A sorted set of many different files
/// containing their collected dates.
pub struct FileAggregateDateSet {
   files : sorted_vec::SortedSet<FileDateSet>,
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

///////////////////////
// Methods - DateSet //
///////////////////////

impl DateSet {
   /// Creates a new DateSet from
   /// an unsorted Vec.
   pub fn from(
      data : Vec<crate::date::Date>,
   ) -> Self {
      return Self{
         list  : sorted_vec::SortedSet::from_unsorted(data),
      };
   }

   /// Gets a reference to the underlying
   /// data slice.
   pub fn as_slice<'l>(
      &'l self,
   ) -> &'l [crate::date::Date] {
      return &self.list;
   }
}

/////////////////////////////////////
// Trait implementations - DateSet //
/////////////////////////////////////

impl std::ops::Deref for DateSet {
   type Target = [crate::date::Date];

   fn deref(
      & self,
   ) -> & Self::Target {
      return self.as_slice();
   }
}

impl std::cmp::PartialEq for DateSet {
   fn eq(
      & self,
      other : & Self,
   ) -> bool {
      return self.as_slice().eq(other.as_slice());
   }
}

impl std::cmp::PartialOrd for DateSet {
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

impl std::cmp::Eq for DateSet {
}

impl std::cmp::Ord for DateSet {
   fn cmp(
      & self,
      other : & Self,
   ) -> std::cmp::Ordering {
      use std::cmp::Ordering::*;

      // Safety check if one or both have length 0
      if self.is_empty() && other.is_empty() {
         return Equal;
      }
      if self.is_empty() && !other.is_empty() {
         return Less;
      }
      if !self.is_empty() && other.is_empty() {
         return Greater;
      }

      return self.partial_cmp(other).unwrap();
   }
}

///////////////////////////
// Methods - FileDateSet //
///////////////////////////

impl FileDateSet {
   /// Creates a new FileDateSet from
   /// an existing path buffer and
   /// date set.
   pub fn from(
      path  : std::path::PathBuf,
      dates : DateSet,
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
   ) -> &'l DateSet {
      return &self.dates;
   }
}

/////////////////////////////////////////
// Trait implementations - FileDateSet //
/////////////////////////////////////////

impl std::cmp::PartialEq for FileDateSet {
   fn eq(
      & self,
      other : & Self,
   ) -> bool {
      return self.path().eq(other.path()) && self.dates().eq(other.dates());
   }
}

impl std::cmp::PartialOrd for FileDateSet {
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

impl std::cmp::Eq for FileDateSet {
}

impl std::cmp::Ord for FileDateSet {
   fn cmp(
      & self,
      other : & Self,
   ) -> std::cmp::Ordering {
      return self.partial_cmp(other).unwrap();
   }
}

/////////////////////////////////////////////
// Internal helpers - FileAggregateDateSet //
/////////////////////////////////////////////

impl FileAggregateDateSet {
   /// Searches an input text stream for
   /// dates and return a DateSet containing
   /// the found dates.
   fn internal_collect_dates(
      text  : & str,
   ) -> DateSet {
      // Get an unsorted list of dates
      let dates = crate::date::Date::from_text_multi(text);

      // Sort and remove duplicates
      let dates = DateSet::from(dates);

      // Return success
      return dates;
   }

   /// Searches a single file for dates and returns
   /// a DateSet containing all the found dates.
   fn internal_search_file_single<F>(
      path     : & std::path::Path,
      per_file : F,
   ) -> Result<DateSet>
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
      file_set_buffer   : & mut Vec<FileDateSet>,
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
               file_set_buffer.push(FileDateSet::from(
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
                  file_set_buffer,
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

