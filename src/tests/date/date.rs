//! Unit tests for crate::date::Date.

#[test]
fn methods() {
   use crate::date::{Date, Month::*};

   let d01 = Date::new(1,  January,    2000, ).unwrap();
   let d02 = Date::new(28, February,   2022, ).unwrap();
   let d03 = Date::new(29, February,   2022, );
   let d04 = Date::new(28, February,   2020, ).unwrap();
   let d05 = Date::new(29, February,   2020, ).unwrap();
   let d06 = Date::new(31, January,    2000, ).unwrap();
   let d07 = Date::new(32, January,    2000, );
   let d08 = Date::new(0,  January,    2000, );
   let d09 = Date::new(29, February,   1900, );
   let d10 = Date::new(29, February,   1992, ).unwrap();
   let d11 = Date::new(29, February,   2000, ).unwrap();

   let d12 = unsafe{Date::new_unchecked(29,  February,   1900, )};
   let d13 = unsafe{Date::new_unchecked(29,  February,   1992, )};
   let d14 = unsafe{Date::new_unchecked(29,  February,   2000, )};
   let d15 = unsafe{Date::new_unchecked(0,   January,    0,    )};
   let d16 = unsafe{Date::new_unchecked(32,  January,    0,    )};

   assert!(
      d01.day_of_month()   == 1        &&
      d01.month()          == January  &&
      d01.year()           == 2000
   );
   assert!(
      d02.day_of_month()   == 28       &&
      d02.month()          == February &&
      d02.year()           == 2022
   );
   assert!(
      d03.is_err() == true,
   );
   assert!(
      d04.day_of_month()   == 28       &&
      d04.month()          == February &&
      d04.year()           == 2020
   );
   assert!(
      d05.day_of_month()   == 29       &&
      d05.month()          == February &&
      d05.year()           == 2020
   );
   assert!(
      d06.day_of_month()   == 31       &&
      d06.month()          == January  &&
      d06.year()           == 2000
   );
   assert!(
      d07.is_err() == true,
   );
   assert!(
      d08.is_err() == true,
   );
   assert!(
      d09.is_err() == true,
   );
   assert!(
      d10.day_of_month()   == 29       &&
      d10.month()          == February &&
      d10.year()           == 1992
   );
   assert!(
      d11.day_of_month()   == 29       &&
      d11.month()          == February &&
      d11.year()           == 2000
   );
   assert!(
      d12.day_of_month()   == 29       &&
      d12.month()          == February &&
      d12.year()           == 1900
   );
   assert!(
      d13.day_of_month()   == 29       &&
      d13.month()          == February &&
      d13.year()           == 1992
   );
   assert!(
      d14.day_of_month()   == 29       &&
      d14.month()          == February &&
      d14.year()           == 2000
   );
   assert!(
      d15.day_of_month()   == 0        &&
      d15.month()          == January  &&
      d15.year()           == 0
   );
   assert!(
      d16.day_of_month()   == 32       &&
      d16.month()          == January  &&
      d16.year()           == 0
   );

   return;
}

#[test]
fn from_string() {
   use crate::date::{Date, Month::*};

   let s00 = r"
      This is some random unrelated text!
      Creation date: December 20th, 2022
      Revision date: January   7th, 2023
      This is some more random unrelated text!
      Blah blah blah...
      My birthday:   November 28, 2003
   ";
   
   let r00 = vec![
      Date::new(20, December, 2022).unwrap(),
      Date::new( 7, January,  2023).unwrap(),
      Date::new(28, November, 2003).unwrap(),
   ];

   assert!(Date::from_string(s00) == r00);

   return;
}

#[test]
fn trait_std_cmp_partialeq() {
   use crate::date::{Date, Month::*};
   
   let d01 = Date::new(1,  January,    2000, ).unwrap();
   let d02 = Date::new(1,  January,    2000, ).unwrap();
   let d03 = Date::new(2,  January,    2000, ).unwrap();
   let d04 = Date::new(31, January,    2000, ).unwrap();
   let d05 = Date::new(28, February,   2000, ).unwrap();
   let d06 = Date::new(1,  January,    912,  ).unwrap();
   let d07 = Date::new(1,  January,    80,   ).unwrap();
   let d08 = Date::new(30, November,   33,   ).unwrap();
   let d09 = Date::new(30, November,   -33,  ).unwrap();
   let d10 = Date::new(30, November,   -33,  ).unwrap();

   assert!(d01 == d01);
   assert!(d01 == d02);
   assert!(d01 != d03);
   assert!(d01 != d04);
   assert!(d01 != d05);
   assert!(d01 != d06);
   assert!(d01 != d07);
   assert!(d01 != d08);
   assert!(d01 != d09);
   assert!(d01 != d10);

   assert!(d02 == d01);
   assert!(d02 == d02);
   assert!(d02 != d03);
   assert!(d02 != d04);
   assert!(d02 != d05);
   assert!(d02 != d06);
   assert!(d02 != d07);
   assert!(d02 != d08);
   assert!(d02 != d09);
   assert!(d02 != d10);

   assert!(d03 != d01);
   assert!(d03 != d02);
   assert!(d03 == d03);
   assert!(d03 != d04);
   assert!(d03 != d05);
   assert!(d03 != d06);
   assert!(d03 != d07);
   assert!(d03 != d08);
   assert!(d03 != d09);
   assert!(d03 != d10);

   assert!(d04 != d01);
   assert!(d04 != d02);
   assert!(d04 != d03);
   assert!(d04 == d04);
   assert!(d04 != d05);
   assert!(d04 != d06);
   assert!(d04 != d07);
   assert!(d04 != d08);
   assert!(d04 != d09);
   assert!(d04 != d10);

   assert!(d05 != d01);
   assert!(d05 != d02);
   assert!(d05 != d03);
   assert!(d05 != d04);
   assert!(d05 == d05);
   assert!(d05 != d06);
   assert!(d05 != d07);
   assert!(d05 != d08);
   assert!(d05 != d09);
   assert!(d05 != d10);

   assert!(d06 != d01);
   assert!(d06 != d02);
   assert!(d06 != d03);
   assert!(d06 != d04);
   assert!(d06 != d05);
   assert!(d06 == d06);
   assert!(d06 != d07);
   assert!(d06 != d08);
   assert!(d06 != d09);
   assert!(d06 != d10);

   assert!(d07 != d01);
   assert!(d07 != d02);
   assert!(d07 != d03);
   assert!(d07 != d04);
   assert!(d07 != d05);
   assert!(d07 != d06);
   assert!(d07 == d07);
   assert!(d07 != d08);
   assert!(d07 != d09);
   assert!(d07 != d10);

   assert!(d08 != d01);
   assert!(d08 != d02);
   assert!(d08 != d03);
   assert!(d08 != d04);
   assert!(d08 != d05);
   assert!(d08 != d06);
   assert!(d08 != d07);
   assert!(d08 == d08);
   assert!(d08 != d09);
   assert!(d08 != d10);

   assert!(d09 != d01);
   assert!(d09 != d02);
   assert!(d09 != d03);
   assert!(d09 != d04);
   assert!(d09 != d05);
   assert!(d09 != d06);
   assert!(d09 != d07);
   assert!(d09 != d08);
   assert!(d09 == d09);
   assert!(d09 == d10);

   assert!(d10 != d01);
   assert!(d10 != d02);
   assert!(d10 != d03);
   assert!(d10 != d04);
   assert!(d10 != d05);
   assert!(d10 != d06);
   assert!(d10 != d07);
   assert!(d10 != d08);
   assert!(d10 == d09);
   assert!(d10 == d10);

   return;
}

#[test]
fn trait_std_cmp_partialord() {
   use crate::date::{Date, Month::*};
   
   let d1 = Date::new(1,   January,    2000, ).unwrap();
   let d2 = Date::new(2,   January,    2000, ).unwrap();
   let d3 = Date::new(31,  January,    2000, ).unwrap();
   let d4 = Date::new(28,  February,   2000, ).unwrap();
   let d5 = Date::new(1,   January,    912,  ).unwrap();
   let d6 = Date::new(1,   January,    80,   ).unwrap();
   let d7 = Date::new(30,  November,   33,   ).unwrap();
   let d8 = Date::new(30,  November,   -33,  ).unwrap();

   assert!(d1 < d2);
   assert!(d1 < d3);
   assert!(d1 < d4);
   assert!(d1 > d5);
   assert!(d1 > d6);
   assert!(d1 > d7);
   assert!(d1 > d8);
   
   assert!(d2 > d1);
   assert!(d2 < d3);
   assert!(d2 < d4);
   assert!(d2 > d5);
   assert!(d2 > d6);
   assert!(d2 > d7);
   assert!(d2 > d8);

   assert!(d3 > d1);
   assert!(d3 > d2);
   assert!(d3 < d4);
   assert!(d3 > d5);
   assert!(d3 > d6);
   assert!(d3 > d7);
   assert!(d3 > d8);

   assert!(d4 > d1);
   assert!(d4 > d2);
   assert!(d4 > d3);
   assert!(d4 > d5);
   assert!(d4 > d6);
   assert!(d4 > d7);
   assert!(d4 > d8);

   assert!(d5 < d1);
   assert!(d5 < d2);
   assert!(d5 < d3);
   assert!(d5 < d4);
   assert!(d5 > d6);
   assert!(d5 > d7);
   assert!(d5 > d8);

   assert!(d6 < d1);
   assert!(d6 < d2);
   assert!(d6 < d3);
   assert!(d6 < d4);
   assert!(d6 < d5);
   assert!(d6 > d7);
   assert!(d6 > d8);

   assert!(d7 < d1);
   assert!(d7 < d2);
   assert!(d7 < d3);
   assert!(d7 < d4);
   assert!(d7 < d5);
   assert!(d7 < d6);
   assert!(d7 > d8);

   assert!(d8 < d1);
   assert!(d8 < d2);
   assert!(d8 < d3);
   assert!(d8 < d4);
   assert!(d8 < d5);
   assert!(d8 < d6);
   assert!(d8 < d7);

   return;
}

#[test]
fn trait_std_fmt_display() {
   use crate::date::{Date, Month::*};

   let s01 = Date::new(1,  January,    2000, ).unwrap().to_string();
   let s02 = Date::new(2,  January,    2000, ).unwrap().to_string();
   let s03 = Date::new(3,  January,    2000, ).unwrap().to_string();
   let s04 = Date::new(4,  January,    2000, ).unwrap().to_string();
   let s05 = Date::new(31, January,    2000, ).unwrap().to_string();
   let s06 = Date::new(28, February,   2000, ).unwrap().to_string();
   let s07 = Date::new(1,  January,    912,  ).unwrap().to_string();
   let s08 = Date::new(1,  January,    80,   ).unwrap().to_string();
   let s09 = Date::new(30, November,   33,   ).unwrap().to_string();
   let s10 = Date::new(30, November,   -33,  ).unwrap().to_string();

   assert!(s01 == "January 1st, 2000"     );
   assert!(s02 == "January 2nd, 2000"     );
   assert!(s03 == "January 3rd, 2000"     );
   assert!(s04 == "January 4th, 2000"     );
   assert!(s05 == "January 31st, 2000"    );
   assert!(s06 == "February 28th, 2000"   );
   assert!(s07 == "January 1st, 912"      );
   assert!(s08 == "January 1st, 80"       );
   assert!(s09 == "November 30th, 33"     );
   assert!(s10 == "November 30th, 33 BCE" );

   return;
}

#[test]
fn trait_std_str_fromstr() {
   use crate::date::{Date, Month::*};
   
   assert!("February 1, 2000"       .parse::<Date>().unwrap() == Date::new(1, February,   2000  ).unwrap());
   assert!("February 1st, 2000"     .parse::<Date>().unwrap() == Date::new(1, February,   2000  ).unwrap());

   assert!(""           .parse::<Date>().is_err() == true);
   assert!("     "      .parse::<Date>().is_err() == true);
   assert!("1st, 2000"  .parse::<Date>().is_err() == true);
   assert!("January 1st".parse::<Date>().is_err() == true);
   assert!("20"         .parse::<Date>().is_err() == true);
   assert!("November"   .parse::<Date>().is_err() == true);

   return;
}

