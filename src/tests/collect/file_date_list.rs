//! Unit tests for crate::collect::FileDateList.

#[test]
fn methods() {
   use std::path::PathBuf;
   use crate::{
      date::{
         Date,
         Month::*,
      },
      collect::{
         FileDateList,
         DateList,
      },
   };

   let mut path = PathBuf::new();
   path.push("launch_codes");
   path.push(".txt");

   let dates = vec![
      Date::new(01, January,  2000).unwrap(),
      Date::new(24, December, 1995).unwrap(),
   ];

   let f0 = FileDateList::from(
      path.clone(),
      DateList::from(dates.clone()),
   );

   assert!(f0.path()    == &path);
   assert!(f0.dates()   == &DateList::from(dates));

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
         FileDateList,
         DateList,
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

   let f0 = FileDateList::from(
      p0,
      DateList::from(vec![
         Date::new(01, January,  2000).unwrap(),
         Date::new(05, January,  2004).unwrap(),
      ]),
   );
   let f1 = FileDateList::from(
      p1,
      DateList::from(vec![
         Date::new(01, January,  2000).unwrap(),
         Date::new(05, January,  2004).unwrap(),
      ]),
   );
   let f2 = FileDateList::from(
      p2,
      DateList::from(vec![
         Date::new(02, January,  2000).unwrap(),
         Date::new(21, March,    2002).unwrap(),
         Date::new(05, January,  2004).unwrap(),
      ]),
   );
   let f3 = FileDateList::from(
      p3,
      DateList::from(vec![
         Date::new(01, January,  2000).unwrap(),
         Date::new(05, April,    2004).unwrap(),
      ]),
   );
   let f4 = FileDateList::from(
      p4,
      DateList::from(vec![
         Date::new(29, December, 1999).unwrap(),
         Date::new(05, January,  2004).unwrap(),
      ]),
   );
   let f5 = FileDateList::from(
      p5,
      DateList::from(vec![
         Date::new(01, January,  2000).unwrap(),
         Date::new(30, January,  2001).unwrap(),
      ]),
   );
   let f6 = FileDateList::from(
      p6,
      DateList::from(Vec::new()),
   );
   let f7 = FileDateList::from(
      p7,
      DateList::from(Vec::new()),
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

