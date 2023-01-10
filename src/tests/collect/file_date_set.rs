//! Unit tests for crate::collect::FileDateSet.

#[test]
fn methods() {
   use std::path::PathBuf;
   use crate::{
      date::{
         Date,
         Month::*,
      },
      collect::{
         FileDateSet,
         DateSet,
      },
   };

   let mut path = PathBuf::new();
   path.push("launch_codes");
   path.push(".txt");

   let dates = vec![
      Date::new(01, January,  2000).unwrap(),
      Date::new(24, December, 1995).unwrap(),
   ];

   let f0 = FileDateSet::from(
      path.clone(),
      DateSet::from(dates.clone()),
   );

   assert!(f0.path()    == &path);
   assert!(f0.dates()   == &DateSet::from(dates));

   return;
}

#[test]
fn trait_std_cmp_ord() {
   use std::{
      cmp::Ordering::*,
      path::PathBuf,
   };
   use crate::{
      date::{
         Date,
         Month::*,
      },
      collect::{
         FileDateSet,
         DateSet,
      },
   };

   let mut p0 = PathBuf::new();
   let mut p1 = PathBuf::new();
   let mut p2 = PathBuf::new();
   let mut p3 = PathBuf::new();
   let mut p4 = PathBuf::new();
   let mut p5 = PathBuf::new();
   let mut p6 = PathBuf::new();
   let mut p7 = PathBuf::new();

   p0.push("foo1.txt");
   p1.push("foo2.txt");
   p2.push("foo3.txt");
   p3.push("foo4.txt");
   p4.push("foo5.txt");
   p5.push("foo6.txt");
   p6.push("foo7.txt");
   p7.push("foo7.txt");

   let f0 = FileDateSet::from(
      p0,
      DateSet::from(vec![
         Date::new(01, January,  2000).unwrap(),
         Date::new(05, January,  2004).unwrap(),
      ]),
   );
   let f1 = FileDateSet::from(
      p1,
      DateSet::from(vec![
         Date::new(01, January,  2000).unwrap(),
         Date::new(05, January,  2004).unwrap(),
      ]),
   );
   let f2 = FileDateSet::from(
      p2,
      DateSet::from(vec![
         Date::new(02, January,  2000).unwrap(),
         Date::new(21, March,    2002).unwrap(),
         Date::new(05, January,  2004).unwrap(),
      ]),
   );
   let f3 = FileDateSet::from(
      p3,
      DateSet::from(vec![
         Date::new(01, January,  2000).unwrap(),
         Date::new(05, April,    2004).unwrap(),
      ]),
   );
   let f4 = FileDateSet::from(
      p4,
      DateSet::from(vec![
         Date::new(29, December, 1999).unwrap(),
         Date::new(05, January,  2004).unwrap(),
      ]),
   );
   let f5 = FileDateSet::from(
      p5,
      DateSet::from(vec![
         Date::new(01, January,  2000).unwrap(),
         Date::new(30, January,  2001).unwrap(),
      ]),
   );
   let f6 = FileDateSet::from(
      p6,
      DateSet::from(Vec::new()),
   );
   let f7 = FileDateSet::from(
      p7,
      DateSet::from(Vec::new()),
   );

   assert!(f0.cmp(&f0) == Equal  );
   assert!(f0.cmp(&f1) == Less   );
   assert!(f0.cmp(&f2) == Less   );
   assert!(f0.cmp(&f3) == Less   );
   assert!(f0.cmp(&f4) == Greater);
   assert!(f0.cmp(&f5) == Greater);
   assert!(f0.cmp(&f6) == Greater);
   assert!(f6.cmp(&f0) == Less   );
   assert!(f6.cmp(&f7) == Equal  );

   return;
}

