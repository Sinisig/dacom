//! Date utilities

/////////////////////////////////
// Struct and enum definitions //
/////////////////////////////////

/// Error type detailing a parsing error for Month.
#[derive(Copy, Clone, Debug)]
pub enum ParseMonthError {
   /// A month matching the input text was not found.
   NoMatchingMonth,
}

/// Error type detailing a parsing error for Date.
#[derive(Copy, Clone, Debug)]
pub enum ParseDateError {
   /// The input string is not formatted as a date.
   InvalidFormatting,

   /// The input day is not a number.
   InvalidDayFormatting,

   /// The input month is not a month.
   InvalidMonthFormatting,

   /// The input year is not a year.
   InvalidYearFormatting,

   /// The day of the month is not valid for the given month and year.
   InvalidDayOfMonth,

   /// The given regex does not have M/D/Y captures.
   InvalidRegexCaptures,
}

/// Enum for storing a month.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Month {
   January,
   February,
   March,
   April,
   May,
   June,
   July,
   August,
   September,
   October,
   November,
   December,
}

// Struct for storing a date.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Date {
   day   : usize,
   month : Month,
   year  : isize,
}

/////////////////////////////////////////////
// Trait implementations - ParseMonthError //
/////////////////////////////////////////////

impl std::fmt::Display for ParseMonthError {
   fn fmt(&self, stream : & mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      return write!(stream, "{}", match self {
         Self::NoMatchingMonth   => "No matching month",
      });
   }
}

impl std::error::Error for ParseMonthError {
}

/////////////////////
// Methods - Month //
/////////////////////

impl Month {
   /// Retrieves the amount of days in the given
   /// month, accounting for leap years.
   pub fn days(&self, is_leap_year : bool) -> usize {
      return match self {
         Self::January     => 31,
         Self::February    => if is_leap_year { 29 } else { 28 },
         Self::March       => 31,
         Self::April       => 30,
         Self::May         => 31,
         Self::June        => 30,
         Self::July        => 31,
         Self::August      => 31,
         Self::September   => 30,
         Self::October     => 31,
         Self::November    => 30,
         Self::December    => 31,
      };
   }
}

///////////////////////////////////
// Trait implementations - Month //
///////////////////////////////////

impl std::fmt::Display for Month {
   fn fmt(&self, stream : & mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      return write!(stream, "{}", match self {
         Self::January     => "January",
         Self::February    => "February",
         Self::March       => "March",
         Self::April       => "April",
         Self::May         => "May",
         Self::June        => "June",
         Self::July        => "July",
         Self::August      => "August",
         Self::September   => "September",
         Self::October     => "October",
         Self::November    => "November",
         Self::December    => "December",
      });
   }
}

impl std::str::FromStr for Month {
   type Err = ParseMonthError;

   fn from_str(string : & str) -> Result<Self, Self::Err> {
      // Slightly dubious because of the string copy.
      let string = string.to_lowercase();

      // A little hideous and runs slow, replace with something more intelligent.
      if string == "january"     || string == "jan" || string == "jan." {
         return Ok(Month::January);
      }
      if string == "february"    || string == "feb" || string == "feb." {
         return Ok(Month::February);
      }
      if string == "march"       || string == "mar" || string == "mar." {
         return Ok(Month::March);
      }
      if string == "april"       || string == "apr" || string == "apr." {
         return Ok(Month::April);
      }
      if string == "may" {
         return Ok(Month::May);
      }
      if string == "june"        || string == "jun" || string == "jun." {
         return Ok(Month::June);
      }
      if string == "july"        || string == "jul" || string == "jul." {
         return Ok(Month::July);
      }
      if string == "august"      || string == "aug" || string == "aug." {
         return Ok(Month::August);
      }
      if string == "september"   || string == "sep" || string == "sep." {
         return Ok(Month::September);
      }
      if string == "october"     || string == "oct" || string == "oct." {
         return Ok(Month::October);
      }
      if string == "november"    || string == "nov" || string == "nov." {
         return Ok(Month::November);
      }
      if string == "december"    || string == "dec" || string == "dec." {
         return Ok(Month::December);
      }

      return Err(ParseMonthError::NoMatchingMonth);
   }
}

////////////////////////////////////////////
// Trait implementations - ParseDateError //
////////////////////////////////////////////

impl std::fmt::Display for ParseDateError {
   fn fmt(&self, stream : & mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      return write!(stream, "{}", match self {
         Self::InvalidFormatting
            => "Text is not a date",
         Self::InvalidDayFormatting
            => "Day is not valid",
         Self::InvalidMonthFormatting
            => "Month is not valid",
         Self::InvalidYearFormatting
            => "Year is not valid",
         Self::InvalidDayOfMonth
            => "Invalid day of month",
         Self::InvalidRegexCaptures
            => "Regex does not contain $m, $d, or $y capture groups",
      });
   }
}

impl std::error::Error for ParseDateError {
}

////////////////////
// Methods - Date //
////////////////////

impl Date {
   /// Attempts to parse a single date from
   /// a text string using a pre-made regular
   /// expression.  This regular expression
   /// looks for dates using the format "
   /// Month Day(th), Year".  See from_text_single_with
   /// for further documentation.
   pub fn from_text_single(
      text  : & str,
   ) -> Result<Self, ParseDateError> {
      use regex::Regex;

      lazy_static::lazy_static!{
         static ref RX_DEFAULT : Regex = Regex::new(r"(?x)
            ^                                      # Start of string
            (?P<m>[[:alpha:]]+)\.?\s*              # Month
            (?P<d>\d{1,2})(?:st|nd|rd|th)?\s*,?\s* # Day
            (?P<y>[+-]?\d+)                        # Year
            $                                      # End of string
         ").unwrap();
      }

      return Self::from_text_single_with(
         text, &RX_DEFAULT,
      );
   }

   /// Searches for a single date within
   /// a text string using a given regular
   /// expression.  The regular expression
   /// must be compiled first by the caller,
   /// then can be passed by reference.  The
   /// regular expression must contain three
   /// named capture groups: $d, $m, and $y.
   /// The day text is represented by $d.  The
   /// month text is represented by $m.  The
   /// year text is represented by $y.  If
   /// the regular expression is missing one
   /// or more of these named capture groups,
   /// and error will be returned when the
   /// regular expression successfully matches
   /// a text string and begins to parse the
   /// day, month, and year captures.
   pub fn from_text_single_with(
      text  : & str,
      regex : & regex::Regex,
   ) -> Result<Self, ParseDateError> {
      use ParseDateError::*;

      // Attempt to run the regex parser and get the captures
      let captures = regex.captures(text).ok_or(InvalidFormatting)?;

      // Get captures for month, day, and year
      let day     = captures.name("d").ok_or(InvalidRegexCaptures)?.as_str();
      let month   = captures.name("m").ok_or(InvalidRegexCaptures)?.as_str();
      let year    = captures.name("y").ok_or(InvalidRegexCaptures)?.as_str();

      // Parse the found month, day, and year
      let day     = day    .parse().map_err(|_| InvalidDayFormatting   )?;
      let month   = month  .parse().map_err(|_| InvalidMonthFormatting )?;
      let year    = year   .parse().map_err(|_| InvalidYearFormatting  )?;

      // Attempt to create a new Date struct from the parsed information
      let date = Self::new(day, month, year)?;

      // Return success
      return Ok(date);
   }

   /// Searches an entire text string for matching
   /// dates and attempts to parse them into a Date
   /// struct, which is then stored inside of a
   /// sorted vector from oldest to newest using
   /// a pre-made regular expression.  The given
   /// regular expression searches for dates in
   /// Month Day(th), Year format.
   pub fn from_text_multi_sorted(
      text  : & str,
   ) -> sorted_vec::SortedVec<Self> {
      use regex::Regex;
      
      lazy_static::lazy_static!{
         static ref RX_DEFAULT : Regex = Regex::new(r"(?x)
            (?P<m>[[:alpha:]]+)\.?\s*              # Month
            (?P<d>\d{1,2})(?:st|nd|rd|th)?\s*,?\s* # Day
            (?P<y>[+-]?\d+)                        # Year
         ").unwrap();
      }

      return Self::from_text_multi_sorted_with(
         text, &RX_DEFAULT,
      ).unwrap(); // If we return Err, the above regex is bugged
   }

   /// Searches an entire text string for dates
   /// matching a given regular expression and
   /// parses them into a Date struct which is
   /// then sorted inside a sorted vector from
   /// oldest to newest.  The regular expression
   /// must contain three named capture groups:
   /// $d, $m, and $y. The day text is represented
   /// by $d.  The month text is represented by $m.
   /// The year text is represented by $y.  If
   /// the regular expression is missing one
   /// or more of these named capture groups,
   /// and error will be returned when the
   /// regular expression successfully matches
   /// a text string and begins to parse the
   /// day, month, and year captures.
   pub fn from_text_multi_sorted_with(
      text  : & str,
      regex : & regex::Regex,
   ) -> Result<sorted_vec::SortedVec<Self>, ParseDateError> {
      use ParseDateError::*;

      let mut dates = sorted_vec::SortedVec::new();
      for cap in regex.captures_iter(text) {
         // Get string slices to captures
         let day     = cap.name("d").ok_or(InvalidRegexCaptures)?.as_str();
         let month   = cap.name("m").ok_or(InvalidRegexCaptures)?.as_str();
         let year    = cap.name("y").ok_or(InvalidRegexCaptures)?.as_str();
         
         // Parse matched text into day, month, year
         let day = match day.parse() {
            Ok(d)    => d,
            Err(_)   => continue,
         };
         let month = match month.parse() {
            Ok(m)    => m,
            Err(_)   => continue,
         };
         let year = match year.parse() {
            Ok(y)    => y,
            Err(_)   => continue,
         };

         // Try to create a new Date
         let date = match Self::new(day, month, year) {
            Ok(d)    => d,
            Err(_)   => continue,
         };

         // Add the date to the list
         dates.push(date);
      }

      // Return successfully
      return Ok(dates);
   }

   /// Creates a new Date object.  Negative
   /// years are considered as dates Before
   /// Common Era (BCE), otherwise known as
   /// Before Christ (BC).  Positive years are
   /// considered as dates during Common Era (CE),
   /// otherwise known as Anno Domini (AD).
   /// If the day of the month is not contained
   /// within the month, an error is returned.
   pub fn new(
      day_of_month   : usize,
      month          : Month,
      year           : isize,
   ) -> Result<Self, ParseDateError> {
      let is_leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
      if day_of_month < 1 || day_of_month > month.days(is_leap_year) {
         return Err(ParseDateError::InvalidDayOfMonth);
      } 

      return Ok(unsafe{Self::new_unchecked(day_of_month, month, year)});
   }

   /// Creates a new Date object without
   /// validating input.  This can result
   /// in runtime errors and hidden bugs
   /// and should never be reasonably used
   /// unless invalid dates are allowable.
   /// See Date::new() for further documentation.
   pub unsafe fn new_unchecked(
      day_of_month   : usize,
      month          : Month,
      year           : isize,
   ) -> Self {
      return Self{
         day   : day_of_month,
         month : month,
         year  : year,
      };
   }

   /// Gets the stored day of the month.
   pub fn day_of_month(&self) -> usize {
      return self.day.clone();
   }

   /// Gets the stored month.
   pub fn month(&self) -> Month {
      return self.month.clone();
   }

   /// Gets the stored year.
   pub fn year(&self) -> isize {
      return self.year.clone();
   }
}

//////////////////////////////////
// Trait implementations - Date //
//////////////////////////////////

impl std::cmp::PartialOrd for Date {
   fn partial_cmp(&self, other : &Self) -> Option<std::cmp::Ordering> {
      if self.year   != other.year {
         return self.year  .partial_cmp(&other.year);
      };
      if self.month  != other.month {
         return self.month .partial_cmp(&other.month);
      }
      if self.day    != other.day {
         return self.day   .partial_cmp(&other.day );
      }

      return Some(std::cmp::Ordering::Equal);
   }
}

impl std::cmp::Ord for Date {
   fn cmp(&self, other : & Self) -> std::cmp::Ordering {
      use std::cmp::Ordering::*; 

      return self.partial_cmp(other).unwrap_or_else(|| Equal);
   }
}

impl std::fmt::Display for Date {
   fn fmt(&self, stream : & mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      return write!(stream,
         "{} {}{}, {}{}",
         self.month,
         self.day,
         if self.day >= 10 && self.day <= 19 {
            "th"
         } else {
            match self.day % 10 {
               1  => "st",
               2  => "nd",
               3  => "rd",
               _  => "th",
            }
         },
         self.year.abs(),
         if self.year >= 0 {
            ""
         } else {
            " BCE"
         },
      );
   }
}

impl std::str::FromStr for Date {
   type Err = ParseDateError;

   fn from_str(text : & str) -> Result<Self, Self::Err> {
      return Self::from_text_single(text);
   }
}

