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
   IOError,
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
      Self::new_recursive_unsorted(
         & mut file_names,
         & mut file_dates,
         String::from(path),
         per_file,
      )?;

      // Sort the files first by oldest date,
      // failing that by newest date, and then
      // failing that by path lexicographically.


      // Return success
      return Ok(Self{
         file_names  : file_names,
         file_dates  : file_dates,
      });
   }

   /// Internal helper for searching a directory
   /// recursively.  The resulting data is unsorted,
   /// which should be fixed with a public wrapper.
   fn new_recursive_unsorted<F>(
      file_name_buffer  : & mut Vec<String>,
      file_date_buffer  : & mut Vec<DateList>,
      path     : String,
      per_file : F,
   ) -> Result<(), FileDateError>
   where F: Fn(& str) + Copy {
      // If the file is a directory, search it recursively
      if match std::fs::metadata(&path) {
         Ok(md)   => md,
         Err(_)   => return Err(FileDateError::IOError),
      }.is_dir() == true {
         for file in match std::fs::read_dir(&path) {
            Ok(itr)  => itr,
            Err(_)   => return Err(FileDateError::IOError),
         } {
            // Unwrap result containing file
            let file = match file {
               Ok(f)    => f,
               Err(_)   => return Err(FileDateError::IOError),
            };
            
            // Create new path
            let path = file
               .path()
               .into_os_string()
               .to_string_lossy()
               .into_owned();

            // Get info from this file
            Self::new_recursive_unsorted(
               file_name_buffer,
               file_date_buffer,
               path,
               per_file,
            )?;
         }
      } else {
         // Run the user closure
         per_file(&path);

         // Read the file as text
         let file = match std::fs::read(&path) {
            Ok(d)    => d,
            Err(_)   => return Err(FileDateError::IOError),
         };
         let file = String::from_utf8_lossy(&file).into_owned();

         // Search for dates within the file
         let mut dates = crate::Date::from_text_multi_sorted_by(&file, |d1, d2| {
            d1.cmp(&d2)
         });

         // Sort the dates
         dates.sort_unstable();

         // Add the results to the vecs
         file_name_buffer.push(path);
         file_date_buffer.push(DateList::from(dates));
      }

      // Return success
      return Ok(());
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
         Self::IOError  => "I/O Error",
      });
   }
}

impl std::error::Error for FileDateError {
}

