//! Date utilities


/* Struct and enum definitions */


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

   /// The day of the month is not valid for the given month and year.
   InvalidDayOfMonth,
}

/// Enum for storing a month.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Date {
   day   : usize,
   month : Month,
   year  : isize,
}


/* Trait implementations - ParseMonthError */


impl std::fmt::Display for ParseMonthError {
   fn fmt(&self, stream : & mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      return write!(stream, "{}", match self {
         Self::NoMatchingMonth   => "No matching month",
      });
   }
}

impl std::error::Error for ParseMonthError {
}


/* Methods - Month */


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


/* Trait implementations - Month */


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


/* Trait implementations - ParseDateError */


impl std::fmt::Display for ParseDateError {
   fn fmt(&self, stream : & mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      return write!(stream, "{}", match self {
         Self::InvalidFormatting => "Text is not a date",
         Self::InvalidDayOfMonth => "Invalid day of month",
      });
   }
}

impl std::error::Error for ParseDateError {
}


/* Methods - Date */


impl Date {
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


/* Trait implementations - Date */


impl std::cmp::PartialOrd for Date {
   fn partial_cmp(&self, other : &Self) -> Option<std::cmp::Ordering> {
      if self.year   != other.year {
         return self.year  .partial_cmp(&other.year);
      };
      if self.month  != other.month {
         return self.month.partial_cmp(&other.month);
      }
      if self.day    != other.day {
         return self.day   .partial_cmp(&other.day );
      }

      return Some(std::cmp::Ordering::Equal);
   }
}

impl std::fmt::Display for Date {
   fn fmt(&self, stream : & mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      return write!(stream,
         "{} {}{}, {}{}",
         self.month,
         self.day,
         match self.day % 10 {
            1  => "st",
            2  => "nd",
            3  => "rd",
            _  => "th",
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

   fn from_str(string : & str) -> Result<Self, Self::Err> {
      use lazy_static::lazy_static;
      use regex::Regex;
      
      // Regex parsers for the many different date formattings, fuck me
      lazy_static!{
         static ref RX_MONTH_DAY_YEAR  : Regex = Regex::new(r"(?x)
            ([[:alpha:]]+)\s*          # Month string
            (\d{1,2})(?:th)?\s*,*\s*   # Day
            (\d{4})                    # Year
         ").unwrap();
         //static ref RX_MMDDYYYY        : Regex = Regex::new(r"(?x)
         //   # TODO: Implement
         //").unwrap();
         //static ref RX_DAY_MONTH_YEAR  : Regex = Regex::new(r"(?x)
         //   # TODO: Implement
         //").unwrap();
         //static ref RX_YYYYMMDD        : Regex = Regex::new(r"(?x)
         //   # TODO: Implement
         //").unwrap();
      }

      return Err(ParseDateError::InvalidFormatting);
   }
}

