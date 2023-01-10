//! Unit tests for crate::collect::CollectDateError.

#[test]
fn trait_std_fmt_display() {
   use crate::collect::CollectDateError::*;

   assert!(PermissionDenied.to_string() == "Permission denied"    );
   assert!(FileNotFound    .to_string() == "File does not exist"  );
   assert!(GeneralIOError  .to_string() == "General I/O error"    );
   assert!(FileIsDirectory .to_string() == "File is a directory"  );
   assert!(InvalidData     .to_string() == "Invalid data"         );

   return;
}

#[test]
fn trait_std_convert_from_std_io_error() {
   use std::io::{Error, ErrorKind};
   use crate::collect::CollectDateError;

   match CollectDateError::from(Error::from(ErrorKind::PermissionDenied )) {
      CollectDateError::PermissionDenied  => (),
      _ => panic!("Unexpected error variant"),
   }
   match CollectDateError::from(Error::from(ErrorKind::NotFound         )) {
      CollectDateError::FileNotFound      => (),
      _ => panic!("Unexpected error variant"),
   }
   match CollectDateError::from(Error::from(ErrorKind::InvalidData      )) {
      CollectDateError::InvalidData       => (),
      _ => panic!("Unexpected error variant"),
   }

   return;
}

