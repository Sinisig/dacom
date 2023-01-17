//! Argument parsing, storing, and access.

use clap::Parser;

/////////////////////////////////
// Struct and enum definitions //
/////////////////////////////////

/// Struct for storing arguments.
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   #[arg(
      short             = 'f',
      long              = "file",
      value_name        = "INPUT_FILE",
      help              = "Input file or directory to be searched",
      required          = true,
   )]
   input_file  : String,

   #[arg(
      short          = 'o',
      long           = "output",
      value_name     = "OUTPUT_FILE",
      help           = "The output file for dating analysis, not specifying prints to stdout",
      required       = false,
   )]
   output_file : Option<String>,

   #[arg(
      short          = 'v',
      long           = "verbose",
      help           = "Enable verbose logging",
      required       = false,
   )]
   verbose     : bool,

   #[arg(
      short          = 't',
      long           = "threads",
      help           = "Amount of threads to use when searching a directory for dates within files",
      required       = false,
   )]
   threads     : Option<std::num::NonZeroUsize>,
}

////////////////////
// Methods - Args //
////////////////////

impl Args {
   /// Creates a new list of arguments from
   /// a list of arguments.
   pub fn new<I, T>(
      argument_list : I
   ) -> Self
   where I: IntoIterator<Item = T>,
         T: Into<std::ffi::OsString> + Clone,
   {
      return Self::parse_from(argument_list);
   }

   /// Retrieves the input file string.
   pub fn input_file<'a>(
      &'a self,
   ) -> &'a str {
      return &self.input_file;
   }

   /// Retrieves the outfile file string.
   pub fn output_file<'a>(
      &'a self,
   ) -> Option<&'a str> {
      return self.output_file.as_ref().map(|s| s.as_str());
   }

   /// Retrieves whether to print verbosely
   /// or not.
   pub fn verbose(
      & self,
   ) -> bool {
      return self.verbose;
   }

   /// Retrieves the specified thread count.
   pub fn threads(
      & self,
   ) -> Option<std::num::NonZeroUsize> {
      return self.threads.clone();
   }
}

