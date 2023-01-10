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
      let list = Self{
         list  : sorted_vec::SortedSet::from_unsorted(data),
      };

      return list;
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

/*

//////////////////////////////////////////
// Internal helpers - FileDateAggregate //
//////////////////////////////////////////

impl FileDateAggregate {
   /// Sorts the input data from oldest
   /// file to newest file.  This is done
   /// by looking at the oldest date from each
   /// file and sorting by that.  If two of
   /// the oldest dates are equal, then the
   /// newest date is used.  If they are
   /// still equal, the file names are used
   /// for the comparison.
   fn internal_sort(
      mut self,
   ) -> Self {
      self.file_info.sort_unstable_by(|(file1, date1), (file2, date2)| {
         use std::cmp::Ordering::*;

         match date1.first().unwrap().cmp(&date2.first().unwrap()) {
            Less     => Less,
            Greater  => Greater,
            Equal    => match date1.last().unwrap().cmp(&date2.last().unwrap()) {
               Less     => Less,
               Greater  => Greater,
               Equal    => file1.cmp(file2),
            },
         }
      });

      return self;
   }

   /// Searches a single file for dates and returns
   /// a DateList containing all the found dates.
   fn internal_collect_single<F>(
      path     : & str,
      per_file : F,
   ) -> Result<DateList, FileDateError>
   where F: Fn(& str) {
      // Execute the user closure
      per_file(path);

      // Check if the file is a directory
      if match std::fs::metadata(path) {
         Ok(md)   => md,
         Err(e)   => return Err(e.into()),
      }.is_dir() == true {
         return Err(FileDateError::FileIsDirectory);
      }

      // Map the file into memory as a string slice
      let file = match std::fs::File::open(&path) {
         Ok(f)    => f,
         Err(e)   => return Err(e.into()),
      };
      let file = match unsafe{memmap2::Mmap::map(&file)} {
         Ok(m)    => m,
         Err(e)   => return Err(e.into()),
      };
      let file = match std::str::from_utf8(&file) {
         Ok(d)    => d,
         Err(_)   => return Err(FileDateError::BinaryFile),
      };

      // Search for dates within the file
      let mut date_list = crate::Date::from_text_multi_sorted_by(
         &file,
         |d1, d2| {
            d1.cmp(&d2)
         },
      );
      
      // Sort the date list
      date_list.sort_unstable();

      // Return success
      return Ok(DateList::from(date_list));
   }

   /// Searches a file path recursively for any
   /// and all files containing dates.  If the
   /// file is binary or contains no dates, it
   /// is not added.  If the file is a directory,
   /// it is recursively searched and every file
   /// in the directory is searched.
   fn internal_collect_recursive_unsorted<F>(
      file_info_buffer  : & mut Vec<(String, DateList)>,
      path              : String,
      per_file          : F,
   ) -> Result<(), FileDateError>
   where F: Fn(& str) + Copy {
      // Try to parse the file path, if it errors as a directory, recursively
      // search that directory
      match Self::internal_collect_single(&path, per_file) {
         Ok(data_list)  => {
            // If the data list contains dates, add the file to the buffers
            if data_list.is_empty() == false {
               file_info_buffer.push((path, data_list));
            }
         },
         Err(err)       => {
            // If it's a directory, do nothing
            // If it's binary data, exit early
            // Otherwise return the error
            match err {
               FileDateError::FileIsDirectory
                  => (),
               FileDateError::BinaryFile
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

               // Copy the new directory path into a String
               // TODO: Smarter way of doing this
               let path = file.path().into_os_string().into_string().unwrap();

               // Try to parse the new file/directory
               Self::internal_collect_recursive_unsorted(
                  file_info_buffer,
                  path,
                  per_file,
               )?;
            }
         },
      }

      // Return success
      return Ok(());
   }
}

*/

