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

/// A pool of spawned threads purposed for
/// finding all dates in a given string.
/// Useful for multi-threading the collection
/// of dates from multiple files.  The threads
/// for a given instance will exit when the
/// instance goes out of scope.
pub struct DateFinderThreadPool {
   pipe_send_list    : Vec<std::sync::mpsc::Sender<std::path::PathBuf>>,
   pipe_recv         : std::sync::mpsc::Receiver<Result<FileDateList>>,
   thread_send_next  : usize,
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

      // Ordering for zero-length lists
      if self.list.is_empty() || other.list.is_empty() {
         return self.list.len().cmp(&other.list.len());
      }

      return self.partial_cmp(other).unwrap_or(Equal);
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
   
   /// Creates a new FileDateList from
   /// a path buffer and reads the file
   /// at the location, constructing a
   /// new DateList.
   pub fn from_file(
      path  : std::path::PathBuf,
   ) -> Result<Self> {
      // Check if the file is a directory
      if match std::fs::metadata(&path) {
         Ok(md)   => md,
         Err(e)   => return Err(e.into()),
      }.is_dir() == true {
         return Err(CollectDateError::FileIsDirectory);
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
         Err(_)   => return Err(CollectDateError::InvalidData),
      };

      // Find all dates within the file
      let dates = crate::date::Date::from_text_multi_sorted(file);
      
      // Construct a DateList struct
      let dates = DateList::from(dates);

      // Return success
      return Ok(Self{
         path  : path,
         dates : dates,
      });
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

      return self.partial_cmp(other).unwrap_or(Equal);
   }
}

//////////////////////////////////////////////
// Internal helpers - FileAggregateDateList //
//////////////////////////////////////////////

impl FileAggregateDateList {
   /// Searches a file path for dates in a file,
   /// searching all files recursively in any
   /// directories encountered, running a closure
   /// for each file that will be searched.  Upon
   /// success, the number of files searched is
   /// returned.
   fn internal_search_dir_recursive<F>(
      thread_pool             : & mut DateFinderThreadPool,
      path                    : std::path::PathBuf,
      mut current_file_count  : usize,
      per_file                : F,
   ) -> Result<usize>
   where F: Fn(& std::path::Path) + Copy {
      // Check if the input file is a directory
      if match std::fs::metadata(&path) {
         Ok(md)   => md,
         Err(e)   => return Err(e.into()),
      }.is_dir() {
         // Iterate for every element in the directory
         for path in std::fs::read_dir(&path)? {
            let path = path?.path();

            // Search this file
            current_file_count = Self::internal_search_dir_recursive(
               thread_pool,
               path,
               current_file_count,
               per_file.clone(),
            )?;
         }
      } else {
         // Execute the user closure
         per_file(&path);

         // Send the path to the thread pool to be parsed
         thread_pool.send(path);

         // Increment the file count
         current_file_count += 1;
      }

      // Return success
      return Ok(current_file_count);
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
      thread_pool : & mut DateFinderThreadPool,
      path        : P,
   ) -> Result<Self>
   where P: AsRef<std::path::Path> {
      // Closure does nothing
      return Self::new_recursive_with(
         thread_pool,
         path,
         |_| {},
      );
   }

   /// Executes the same as Self::new_recursive,
   /// but a user closure is executed for each file
   /// that is searched.  The closure is passed the
   /// path to the current file.
   pub fn new_recursive_with<P, F>(
      thread_pool : & mut DateFinderThreadPool,
      path        : P,
      per_file    : F,
   ) -> Result<Self>
   where P: AsRef<std::path::Path>,
         F: Fn(& std::path::Path) + Copy {
      // Convert the path into a PathBuf
      let mut path_buf = std::path::PathBuf::new();
      path_buf.push(path);

      // Create the buffer for holding file date lists
      let mut file_list_buffer = sorted_vec::SortedVec::new();

      // Assign file paths to the thread pool
      let mut expected_file_count = Self::internal_search_dir_recursive(
         thread_pool,
         path_buf,
         0,
         per_file,
      )?;

      // Start populating the file list buffer with results
      while file_list_buffer.len() < expected_file_count {
         let file_dates = match thread_pool.recv() {
            Some(fd) => fd,
            None     => continue,
         };

         // Unwrap error variant
         let file_dates = match file_dates {
            Ok(fd)   => fd,
            Err(e)   => match e {
               CollectDateError::InvalidData
                  => {
                     expected_file_count -= 1;
                     continue;
                  },
               _
                  => return Err(e),
            },
         };

         // If the date list is empty, nix this file
         // from the data
         if file_dates.dates().is_empty() {
            expected_file_count -= 1;
            continue;
         }

         // Add the file dating to the list
         file_list_buffer.push(file_dates);
      }

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

////////////////////////////////////
// Methods - DateFinderThreadPool //
////////////////////////////////////

impl DateFinderThreadPool {
   /// Creates a new thread pool with the
   /// given number of threads.
   pub fn new(
      thread_count   : std::num::NonZeroUsize,
   ) -> Self {
      // Initialize pipes
      let mut pipe_in_send_list = Vec::new();
      let (
         pipe_out_send,
         pipe_out_recv,
      ) = std::sync::mpsc::channel();

      // Creates threads and pipes for each thread
      for _ in 0..thread_count.get() {
         let (
            pipe_in_send,
            pipe_in_recv,
         ) = std::sync::mpsc::channel();
         let pipe_out_send = pipe_out_send.clone();

         pipe_in_send_list.push(pipe_in_send);
         std::thread::spawn(move || {
            let recv = pipe_in_recv;
            let send = pipe_out_send;

            while let Ok(path) = recv.recv() {
               send.send(FileDateList::from_file(path)).unwrap();
            }

            return;
         });
      }

      // Return a new struct instance with the pipes
      return Self{
         pipe_send_list    : pipe_in_send_list,
         pipe_recv         : pipe_out_recv,
         thread_send_next  : 0,
      };
   }

   /// Send a PathBuf to a file to be searched
   /// for dates.
   pub fn send(
      & mut self,
      path  : std::path::PathBuf,
   ) -> & mut Self {
      self.pipe_send_list[self.thread_send_next].send(path).unwrap();

      self.thread_send_next =
         (self.thread_send_next + 1) % self.pipe_send_list.len();
      return self;
   }

   /// Receives a DateList created from a file.
   /// If there are currently no avaliable dates,
   /// None is returned.  It is recommended to keep
   /// a counter of how many files were sent and to
   /// keep requesting data until the requested data
   /// count matches the sent file count.  Otherwise,
   /// dates may be missed.
   pub fn recv(
      & mut self,
   ) -> Option<Result<FileDateList>> {
      use std::sync::mpsc::TryRecvError;

      return match self.pipe_recv.try_recv() {
         Ok(d)    => Some(d),
         Err(e)   => match e {
            TryRecvError::Empty
               => None,
            TryRecvError::Disconnected
               => panic!("Attempted to receive data from broken pipe"),
         },
      }
   }
}

