//! Unit tests for crate::collect::DateList.

#[test]
fn methods() {
   use crate::{
      collect::DateList,
      date::{
         Date,
         Month::*,
      },
   };

   let s0 = vec![
      Date::new(01, January,  2000).unwrap(),
      Date::new(01, February, 2000).unwrap(),
      Date::new(02, January,  2000).unwrap(),
      Date::new(01, January,  2000).unwrap(),
      Date::new(01, January,  1995).unwrap(),
   ];
   let r0 = vec![
      Date::new(01, January,  1995).unwrap(),
      Date::new(01, January,  2000).unwrap(),
      Date::new(01, January,  2000).unwrap(),
      Date::new(02, January,  2000).unwrap(),
      Date::new(01, February, 2000).unwrap(),
   ];

   assert!(DateList::from(s0).as_slice() == &r0);

   return;
}

#[test]
fn trait_std_ops_cmp() {
   use std::cmp::Ordering::*;
   use crate::{
      collect::DateList,
      date::{
         Date,
         Month::*,
      },
   };

   let s0 = DateList::from(vec![
      Date::new(01, January,  2000).unwrap(),
      Date::new(05, January,  2004).unwrap(),

   ]);
   let s1 = DateList::from(vec![
      Date::new(01, January,  2000).unwrap(),
      Date::new(05, January,  2004).unwrap(),
   ]);
   let s2 = DateList::from(vec![
      Date::new(02, January,  2000).unwrap(),
      Date::new(21, March,    2002).unwrap(),
      Date::new(05, January,  2004).unwrap(),
   ]);
   let s3 = DateList::from(vec![
      Date::new(01, January,  2000).unwrap(),
      Date::new(05, April,    2004).unwrap(),

   ]);
   let s4 = DateList::from(vec![
      Date::new(29, December, 1999).unwrap(),
      Date::new(05, January,  2004).unwrap(),
   ]);
   let s5 = DateList::from(vec![
      Date::new(01, January,  2000).unwrap(),
      Date::new(30, January,  2001).unwrap(),
   ]);
   let s6 = DateList::from(Vec::new());
   let s7 = DateList::from(Vec::new());

   assert!(s0.cmp(&s0) == Equal  );
   assert!(s0.cmp(&s1) == Equal  );
   assert!(s0.cmp(&s2) == Less   );
   assert!(s0.cmp(&s3) == Less   );
   assert!(s0.cmp(&s4) == Greater);
   assert!(s0.cmp(&s5) == Greater);
   assert!(s0.cmp(&s6) == Greater);
   assert!(s6.cmp(&s0) == Less   );
   assert!(s6.cmp(&s7) == Equal  );

   return;
}

