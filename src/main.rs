fn main() -> Result<(), Box<dyn std::error::Error>> {
   // Parse command-line arguments
   let args = dacom::Args::new(std::env::args());
   
   // Collect dates from files
   if args.verbose() {println!(
      "Starting search for and collection of dates starting at {}...",
      &args.input_file(),
   )};
   let data = dacom::FileAggregateDateSet::new_recursive_with(
      &args.input_file(),
      if args.verbose() {
         |path : & std::path::Path| {
            println!("Searching {}...", path.to_str().unwrap());
         }
      } else {
         |_ : & std::path::Path| {}
      },
   )?;
   
   // Create a data report
   if args.verbose() {println!(
      "Analyzing found dates and creating a report...",
   )};
   let data = data.create_report();

   // Send the data to the appropriate file stream
   if let Some(path) = &args.output_file() {
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

