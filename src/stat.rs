//! Utilities for aggregating and analyzing data.


/* Struct and enum definitions */


/// A sorted list of dates from oldest to
/// newest.
pub struct DateList {
   list  : Vec<crate::Date>,
}

/// An enum containing an error involving
/// file dating.
#[derive(Copy, Clone, Debug)]
pub enum FileDateError {
   GeneralIOError,
   PermissionDenied,
   FileNotFound,
   FileIsDirectory,
}

/// A single file date collection element created
/// by FileDateAggregateIterator.
pub struct FileDate<'l> {
   file_name   : &'l str,
   date_list   : &'l DateList,
}

/// A collection of files with their dating
/// information.
pub struct FileDateAggregate {
   // Implemented as SoA for better cache performance.
   // Each element of file_names corresponds 1:1 with
   // an element of the same index from file_dates.
   file_names  : Vec<String>,
   file_dates  : Vec<DateList>,
}

/// An iterator over a FileDateAggregate object.
pub struct FileDateAggregateIterator<'l> {
   index : usize,
   data  : &'l FileDateAggregate,
}


/* Methods - DateList */


impl DateList {
   /// Creates a new Date List from an existing
   /// sorted array.
   pub fn from(
      date_list   : Vec<crate::Date>,
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
   pub fn as_ref<'l>(
      &'l self,
   ) -> &'l [crate::Date] {
      return &self.list;
   }
}


/* Trait implementations - DateList */


impl std::ops::Deref for DateList {
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
      date_list   : &'l DateList,
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

/* Internal helpers - FileDateAggregate */

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
      self,
   ) -> Self {
      // TODO: Sorting

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

      // Make sure the file isn't a directory
      if match std::fs::metadata(path) {
         Ok(md)   => md,
         Err(e)   => return Err(e.into()),
      }.is_dir() == true {
         return Err(FileDateError::FileIsDirectory);
      }

      // Read the file as text
      let file = match std::fs::read(&path) {
         Ok(d)    => d,
         Err(e)   => return Err(e.into()),
      };
      let file = String::from_utf8_lossy(&file).into_owned();

      // Search for dates within the file
      let mut date_list = crate::Date::from_text_multi_sorted_by(&file, |d1, d2| {
         d1.cmp(&d2)
      });
      
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
      file_name_buffer  : & mut Vec<String>,
      file_date_buffer  : & mut Vec<DateList>,
      path     : String,
      per_file : F,
   ) -> Result<(), FileDateError>
   where F: Fn(& str) + Copy {
      // Try to parse the file path, if it errors as a directory, recursively
      // search that directory
      match Self::internal_collect_single(&path, per_file) {
         Ok(data_list)  => {
            // If the data list contains dates, add the file to the buffers
            if data_list.is_empty() == false {
               file_name_buffer.push(path);
               file_date_buffer.push(data_list);
            }
         },
         Err(err)       => {
            // If it is not a directory error, return that error
            match err {
               FileDateError::FileIsDirectory => (),
               _ => return Err(err),
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
                  file_name_buffer,
                  file_date_buffer,
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

/* Methods - FileDateAggregate */


impl FileDateAggregate { 
   /// Searches a file for files with dating
   /// information and stores those with found
   /// dates.  Directories are recursively searched.
   /// The input closure is executed for every
   /// file searched.
   pub fn new<F>(
      path     : & str,
      per_file : F,
   ) -> Result<Self, FileDateError>
   where F: Fn(& str) + Copy {
      // Get the unsorted data
      let mut file_names = Vec::new();
      let mut file_dates = Vec::new();
      Self::internal_collect_recursive_unsorted(
         & mut file_names,
         & mut file_dates,
         String::from(path),
         per_file,
      )?;

      // Create the struct and sort it
      let data = Self{
         file_names  : file_names,
         file_dates  : file_dates,
      }.internal_sort();

      // Return success
      return Ok(data);
   }

   /// Creates an iterator over the elements.
   pub fn iter<'l>(
      &'l self,
   ) -> FileDateAggregateIterator<'l> {
      return FileDateAggregateIterator::new(self);
   }

   /// Gets the file count.
   pub fn count(
      & self,
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

      self.index += 1;
      return Some(FileDate::from(
         &self.data.file_names[self.index - 1],
         &self.data.file_dates[self.index - 1],
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
         Self::GeneralIOError =>
            "General I/O error",
         Self::PermissionDenied =>
            "Permission denied",
         Self::FileNotFound =>
            "File not found",
         Self::FileIsDirectory =>
            "File is a directory",
      });
   }
}

impl std::error::Error for FileDateError {
}

impl std::convert::From<std::io::Error> for FileDateError {
   fn from(err : std::io::Error) -> Self {
      return Self::GeneralIOError;
   }
}

