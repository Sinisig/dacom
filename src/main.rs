fn main() {
   use colored::Colorize;
   const EXIT_FAILURE : i32 = 1;

   if let Err(e) = dacom_main() {
      eprintln!("{} {}",
         "Error:".red().bold(),
         e.to_string()
      );
      std::process::exit(EXIT_FAILURE);
   }

   return;
}

fn dacom_main() -> Result<(), Box<dyn std::error::Error>> {
   // Parse command-line arguments
   let args = dacom::Args::new(std::env::args());
  
   // Get thread count, defaulting to CPU core count
   let thread_count = args.threads().unwrap_or(
      std::thread::available_parallelism()?,
   );

   // Compile the input regex or use the default one
   let regex = regex::Regex::new(args.date_regex().unwrap_or(r"(?x)
      ###### Format #######
      Month Day(th)(,) Year
      #####################

      (?P<m>[[:alpha:]])\.?\s*               # Month
      (?P<d>\d{1,2})(?:st|nd|rd|th)?\s*,?\s* # Day
      (?P<y>[+-]?\d+)                        # Year
   "))?;

   // Create the thread pool
   if args.verbose() {println!(
      "Creating a thread pool with {} threads...",
      thread_count,
   )};
   let mut thread_pool = dacom::DateFinderThreadPool::new(thread_count);

   // Collect dates from files
   if args.verbose() {println!(
      "Starting search for and collection of dates starting at {}...",
      args.input_file(),
   )};
   let data = dacom::FileAggregateDateList::new_recursive_with(
      & mut thread_pool,
      args.input_file(),
      |path| if args.verbose() {println!(
         "Searching {}...",
         path.to_str().unwrap_or("(???)"),
      )},
   )?;
   
   // Create a data report
   if args.verbose() {println!(
      "Analyzing found dates and creating a report...",
   )};
   let data = data.create_report()?;

   // Send the data to the appropriate file stream
   if let Some(path) = args.output_file() {
      if args.verbose() {println!(
         "Writing results to {path}...",
      )};
      std::fs::write(path, data.to_string())?;
   } else {
      print!("{data}");
   }

   // Return success
   if args.verbose() {println!(
      "Exiting program successfully...",
   )};
   return Ok(());
}

