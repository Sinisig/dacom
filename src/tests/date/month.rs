//! Unit tests for crate::date::Month.

#[test]
fn days() {
   use crate::date::Month::*;

   assert!(January   .days(false) == 31);
   assert!(February  .days(false) == 28);
   assert!(March     .days(false) == 31);
   assert!(April     .days(false) == 30);
   assert!(May       .days(false) == 31);
   assert!(June      .days(false) == 30);
   assert!(July      .days(false) == 31);
   assert!(August    .days(false) == 31);
   assert!(September .days(false) == 30);
   assert!(October   .days(false) == 31);
   assert!(November  .days(false) == 30);
   assert!(December  .days(false) == 31);

   assert!(January   .days(true) == 31);
   assert!(February  .days(true) == 29);
   assert!(March     .days(true) == 31);
   assert!(April     .days(true) == 30);
   assert!(May       .days(true) == 31);
   assert!(June      .days(true) == 30);
   assert!(July      .days(true) == 31);
   assert!(August    .days(true) == 31);
   assert!(September .days(true) == 30);
   assert!(October   .days(true) == 31);
   assert!(November  .days(true) == 30);
   assert!(December  .days(true) == 31);

   return;
}

#[test]
fn trait_std_fmt_display() {
   use crate::date::Month::*;

   assert!(January   .to_string() == "January",    );
   assert!(February  .to_string() == "February",   );
   assert!(March     .to_string() == "March",      );
   assert!(April     .to_string() == "April",      );
   assert!(May       .to_string() == "May",        );
   assert!(June      .to_string() == "June",       );
   assert!(July      .to_string() == "July",       );
   assert!(August    .to_string() == "August",     );
   assert!(September .to_string() == "September",  );
   assert!(October   .to_string() == "October",    );
   assert!(November  .to_string() == "November",   );
   assert!(December  .to_string() == "December",   );

   return;
}

#[test]
fn trait_std_cmp_partialeq() {
   use crate::date::Month::*;

   assert!(January   == January,    );
   assert!(February  == February,   );
   assert!(March     == March,      );
   assert!(April     == April,      );
   assert!(May       == May,        );
   assert!(June      == June,       );
   assert!(July      == July,       );
   assert!(August    == August,     );
   assert!(September == September,  );
   assert!(October   == October,    );
   assert!(November  == November,   );
   assert!(December  == December,   );
   
   assert!(January != February,  );
   assert!(January != March,     );
   assert!(January != April,     );
   assert!(January != May,       );
   assert!(January != June,      );
   assert!(January != July,      );
   assert!(January != August,    );
   assert!(January != September, );
   assert!(January != October,   );
   assert!(January != November,  );
   assert!(January != December,  );
   
   assert!(February != January,     );
   assert!(February != March,       );
   assert!(February != April,       );
   assert!(February != May,         );
   assert!(February != June,        );
   assert!(February != July,        );
   assert!(February != August,      );
   assert!(February != September,   );
   assert!(February != October,     );
   assert!(February != November,    );
   assert!(February != December,    );
   
   assert!(March != January,     );
   assert!(March != February,    );
   assert!(March != April,       );
   assert!(March != May,         );
   assert!(March != June,        );
   assert!(March != July,        );
   assert!(March != August,      );
   assert!(March != September,   );
   assert!(March != October,     );
   assert!(March != November,    );
   assert!(March != December,    );

   assert!(April != January,     );
   assert!(April != February,    );
   assert!(April != March,       );
   assert!(April != May,         );
   assert!(April != June,        );
   assert!(April != July,        );
   assert!(April != August,      );
   assert!(April != September,   );
   assert!(April != October,     );
   assert!(April != November,    );
   assert!(April != December,    );

   assert!(May != January,    );
   assert!(May != February,   );
   assert!(May != March,      );
   assert!(May != April,      );
   assert!(May != June,       );
   assert!(May != July,       );
   assert!(May != August,     );
   assert!(May != September,  );
   assert!(May != October,    );
   assert!(May != November,   );
   assert!(May != December,   );
   
   assert!(June != January,   );
   assert!(June != February,  );
   assert!(June != March,     );
   assert!(June != April,     );
   assert!(June != May,       );
   assert!(June != July,      );
   assert!(June != August,    );
   assert!(June != September, );
   assert!(June != October,   );
   assert!(June != November,  );
   assert!(June != December,  );
   
   assert!(July != January,   );
   assert!(July != February,  );
   assert!(July != March,     );
   assert!(July != April,     );
   assert!(July != May,       );
   assert!(July != June,      );
   assert!(July != August,    );
   assert!(July != September, );
   assert!(July != October,   );
   assert!(July != November,  );
   assert!(July != December,  );

   assert!(August != January,    );
   assert!(August != February,   );
   assert!(August != March,      );
   assert!(August != April,      );
   assert!(August != May,        );
   assert!(August != June,       );
   assert!(August != July,       );
   assert!(August != September,  );
   assert!(August != October,    );
   assert!(August != November,   );
   assert!(August != December,   );
   
   assert!(September != January,    );
   assert!(September != February,   );
   assert!(September != March,      );
   assert!(September != April,      );
   assert!(September != May,        );
   assert!(September != June,       );
   assert!(September != July,       );
   assert!(September != August,     );
   assert!(September != October,    );
   assert!(September != November,   );
   assert!(September != December,   );

   assert!(October != January,   );
   assert!(October != February,  );
   assert!(October != March,     );
   assert!(October != April,     );
   assert!(October != May,       );
   assert!(October != June,      );
   assert!(October != July,      );
   assert!(October != August,    );
   assert!(October != September, );
   assert!(October != November,  );
   assert!(October != December,  );

   assert!(November != January,     );
   assert!(November != February,    );
   assert!(November != March,       );
   assert!(November != April,       );
   assert!(November != May,         );
   assert!(November != June,        );
   assert!(November != July,        );
   assert!(November != August,      );
   assert!(November != September,   );
   assert!(November != October,     );
   assert!(November != December,    );

   assert!(December != January,     );
   assert!(December != February,    );
   assert!(December != March,       );
   assert!(December != April,       );
   assert!(December != May,         );
   assert!(December != June,        );
   assert!(December != July,        );
   assert!(December != August,      );
   assert!(December != September,   );
   assert!(December != October,     );
   assert!(December != November,    ); 

   return;
}

#[test]
fn trait_std_cmp_partialord() {
   use crate::date::Month::*;

   assert!(January < February,   );
   assert!(January < March,      );
   assert!(January < April,      );
   assert!(January < May,        );
   assert!(January < June,       );
   assert!(January < July,       );
   assert!(January < August,     );
   assert!(January < September,  );
   assert!(January < October,    );
   assert!(January < November,   );
   assert!(January < December,   );
    
   assert!(February > January,   );
   assert!(February < March,     );
   assert!(February < April,     );
   assert!(February < May,       );
   assert!(February < June,      );
   assert!(February < July,      );
   assert!(February < August,    );
   assert!(February < September, );
   assert!(February < October,   );
   assert!(February < November,  );
   assert!(February < December,  );
   
   assert!(March > January,   );
   assert!(March > February,  );
   assert!(March < April,     );
   assert!(March < May,       );
   assert!(March < June,      );
   assert!(March < July,      );
   assert!(March < August,    );
   assert!(March < September, );
   assert!(March < October,   );
   assert!(March < November,  );
   assert!(March < December,  );

   assert!(April > January,   );
   assert!(April > February,  );
   assert!(April > March,     );
   assert!(April < May,       );
   assert!(April < June,      );
   assert!(April < July,      );
   assert!(April < August,    );
   assert!(April < September, );
   assert!(April < October,   );
   assert!(April < November,  );
   assert!(April < December,  );

   assert!(May > January,     );
   assert!(May > February,    );
   assert!(May > March,       );
   assert!(May > April,       );
   assert!(May < June,        );
   assert!(May < July,        );
   assert!(May < August,      );
   assert!(May < September,   );
   assert!(May < October,     );
   assert!(May < November,    );
   assert!(May < December,    );
   
   assert!(June > January,    );
   assert!(June > February,   );
   assert!(June > March,      );
   assert!(June > April,      );
   assert!(June > May,        );
   assert!(June < July,       );
   assert!(June < August,     );
   assert!(June < September,  );
   assert!(June < October,    );
   assert!(June < November,   );
   assert!(June < December,   );
   
   assert!(July > January,    );
   assert!(July > February,   );
   assert!(July > March,      );
   assert!(July > April,      );
   assert!(July > May,        );
   assert!(July > June,       );
   assert!(July < August,     );
   assert!(July < September,  );
   assert!(July < October,    );
   assert!(July < November,   );
   assert!(July < December,   );

   assert!(August > January,     );
   assert!(August > February,    );
   assert!(August > March,       );
   assert!(August > April,       );
   assert!(August > May,         );
   assert!(August > June,        );
   assert!(August > July,        );
   assert!(August < September,   );
   assert!(August < October,     );
   assert!(August < November,    );
   assert!(August < December,    );
   
   assert!(September > January,  );
   assert!(September > February, );
   assert!(September > March,    );
   assert!(September > April,    );
   assert!(September > May,      );
   assert!(September > June,     );
   assert!(September > July,     );
   assert!(September > August,   );
   assert!(September < October,  );
   assert!(September < November, );
   assert!(September < December, );

   assert!(October > January,    );
   assert!(October > February,   );
   assert!(October > March,      );
   assert!(October > April,      );
   assert!(October > May,        );
   assert!(October > June,       );
   assert!(October > July,       );
   assert!(October > August,     );
   assert!(October > September,  );
   assert!(October < November,   );
   assert!(October < December,   );

   assert!(November > January,   );
   assert!(November > February,  );
   assert!(November > March,     );
   assert!(November > April,     );
   assert!(November > May,       );
   assert!(November > June,      );
   assert!(November > July,      );
   assert!(November > August,    );
   assert!(November > September, );
   assert!(November > October,   );
   assert!(November < December,  );

   assert!(December > January,   );
   assert!(December > February,  );
   assert!(December > March,     );
   assert!(December > April,     );
   assert!(December > May,       );
   assert!(December > June,      );
   assert!(December > July,      );
   assert!(December > August,    );
   assert!(December > September, );
   assert!(December > October,   );
   assert!(December > November,  );

   return;
}

#[test]
fn trait_std_str_fromstr() {
   use crate::date::Month::{self, *};

   assert!("January" .parse::<Month>().unwrap() == January);
   assert!("january" .parse::<Month>().unwrap() == January);
   assert!("JANUARY" .parse::<Month>().unwrap() == January);
   assert!("Jan"     .parse::<Month>().unwrap() == January);
   assert!("jan"     .parse::<Month>().unwrap() == January);
   assert!("JAN"     .parse::<Month>().unwrap() == January);
   assert!("Jan."    .parse::<Month>().unwrap() == January);
   assert!("jan."    .parse::<Month>().unwrap() == January);
   assert!("JAN."    .parse::<Month>().unwrap() == January);

   assert!("February".parse::<Month>().unwrap() == February);
   assert!("february".parse::<Month>().unwrap() == February);
   assert!("FEBRUARY".parse::<Month>().unwrap() == February);
   assert!("Feb"     .parse::<Month>().unwrap() == February);
   assert!("feb"     .parse::<Month>().unwrap() == February);
   assert!("FEB"     .parse::<Month>().unwrap() == February);
   assert!("Feb."    .parse::<Month>().unwrap() == February);
   assert!("feb."    .parse::<Month>().unwrap() == February);
   assert!("FEB."    .parse::<Month>().unwrap() == February);

   assert!("March".parse::<Month>().unwrap() == March);
   assert!("march".parse::<Month>().unwrap() == March);
   assert!("MARCH".parse::<Month>().unwrap() == March);
   assert!("Mar"  .parse::<Month>().unwrap() == March);
   assert!("mar"  .parse::<Month>().unwrap() == March);
   assert!("MAR"  .parse::<Month>().unwrap() == March);
   assert!("Mar." .parse::<Month>().unwrap() == March);
   assert!("mar." .parse::<Month>().unwrap() == March);
   assert!("MAR." .parse::<Month>().unwrap() == March);

   assert!("April".parse::<Month>().unwrap() == April);
   assert!("april".parse::<Month>().unwrap() == April);
   assert!("APRIL".parse::<Month>().unwrap() == April);
   assert!("Apr"  .parse::<Month>().unwrap() == April);
   assert!("apr"  .parse::<Month>().unwrap() == April);
   assert!("APR"  .parse::<Month>().unwrap() == April);
   assert!("Apr." .parse::<Month>().unwrap() == April);
   assert!("apr." .parse::<Month>().unwrap() == April);
   assert!("APR." .parse::<Month>().unwrap() == April);

   assert!("May"  .parse::<Month>().unwrap() == May);
   assert!("may"  .parse::<Month>().unwrap() == May);
   assert!("MAY"  .parse::<Month>().unwrap() == May);
   
   assert!("June" .parse::<Month>().unwrap() == June);
   assert!("june" .parse::<Month>().unwrap() == June);
   assert!("JUNE" .parse::<Month>().unwrap() == June);
   assert!("Jun"  .parse::<Month>().unwrap() == June);
   assert!("jun"  .parse::<Month>().unwrap() == June);
   assert!("JUN"  .parse::<Month>().unwrap() == June);
   assert!("Jun." .parse::<Month>().unwrap() == June);
   assert!("jun." .parse::<Month>().unwrap() == June);
   assert!("JUN." .parse::<Month>().unwrap() == June);

   assert!("July" .parse::<Month>().unwrap() == July);
   assert!("july" .parse::<Month>().unwrap() == July);
   assert!("JULY" .parse::<Month>().unwrap() == July);
   assert!("Jul"  .parse::<Month>().unwrap() == July);
   assert!("jul"  .parse::<Month>().unwrap() == July);
   assert!("JUL"  .parse::<Month>().unwrap() == July);
   assert!("Jul." .parse::<Month>().unwrap() == July);
   assert!("jul." .parse::<Month>().unwrap() == July);
   assert!("JUL." .parse::<Month>().unwrap() == July);

   assert!("August"  .parse::<Month>().unwrap() == August);
   assert!("august"  .parse::<Month>().unwrap() == August);
   assert!("AUGUST"  .parse::<Month>().unwrap() == August);
   assert!("Aug"     .parse::<Month>().unwrap() == August);
   assert!("aug"     .parse::<Month>().unwrap() == August);
   assert!("AUG"     .parse::<Month>().unwrap() == August);
   assert!("Aug."    .parse::<Month>().unwrap() == August);
   assert!("aug."    .parse::<Month>().unwrap() == August);
   assert!("AUG."    .parse::<Month>().unwrap() == August);

   assert!("September"  .parse::<Month>().unwrap() == September);
   assert!("september"  .parse::<Month>().unwrap() == September);
   assert!("SEPTEMBER"  .parse::<Month>().unwrap() == September);
   assert!("Sep"        .parse::<Month>().unwrap() == September);
   assert!("sep"        .parse::<Month>().unwrap() == September);
   assert!("SEP"        .parse::<Month>().unwrap() == September);
   assert!("Sep."       .parse::<Month>().unwrap() == September);
   assert!("sep."       .parse::<Month>().unwrap() == September);
   assert!("SEP."       .parse::<Month>().unwrap() == September);

   assert!("October" .parse::<Month>().unwrap() == October);
   assert!("october" .parse::<Month>().unwrap() == October);
   assert!("OCTOBER" .parse::<Month>().unwrap() == October);
   assert!("Oct"     .parse::<Month>().unwrap() == October);
   assert!("oct"     .parse::<Month>().unwrap() == October);
   assert!("OCT"     .parse::<Month>().unwrap() == October);
   assert!("Oct."    .parse::<Month>().unwrap() == October);
   assert!("oct."    .parse::<Month>().unwrap() == October);
   assert!("OCT."    .parse::<Month>().unwrap() == October);

   assert!("November".parse::<Month>().unwrap() == November);
   assert!("november".parse::<Month>().unwrap() == November);
   assert!("NOVEMBER".parse::<Month>().unwrap() == November);
   assert!("Nov"     .parse::<Month>().unwrap() == November);
   assert!("nov"     .parse::<Month>().unwrap() == November);
   assert!("NOV"     .parse::<Month>().unwrap() == November);
   assert!("Nov."    .parse::<Month>().unwrap() == November);
   assert!("nov."    .parse::<Month>().unwrap() == November);
   assert!("NOV."    .parse::<Month>().unwrap() == November);

   assert!("December".parse::<Month>().unwrap() == December);
   assert!("december".parse::<Month>().unwrap() == December);
   assert!("DECEMBER".parse::<Month>().unwrap() == December);
   assert!("Dec"     .parse::<Month>().unwrap() == December);
   assert!("dec"     .parse::<Month>().unwrap() == December);
   assert!("DEC"     .parse::<Month>().unwrap() == December);
   assert!("Dec."    .parse::<Month>().unwrap() == December);
   assert!("dec."    .parse::<Month>().unwrap() == December);
   assert!("DEC."    .parse::<Month>().unwrap() == December);

   assert!(""        .parse::<Month>().is_err() == true);
   assert!("    "    .parse::<Month>().is_err() == true);
   assert!("Octember".parse::<Month>().is_err() == true);
   assert!("Junuary" .parse::<Month>().is_err() == true);
   assert!("JulyJune".parse::<Month>().is_err() == true);
   assert!("  May "  .parse::<Month>().is_err() == true);
   assert!("October.".parse::<Month>().is_err() == true);
   assert!("enuJ"    .parse::<Month>().is_err() == true);

   return;
}

