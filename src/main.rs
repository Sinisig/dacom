fn main() -> Result<(), Box<dyn std::error::Error>> {
   // Parse command-line arguments
   let args = dacom::Args::new(std::env::args());
   
   // Load dates from files
   if args.verbose() {println!(
      "Aggregating file dating information starting at {}...",
      args.input_file(),
   )};
   let data = dacom::FileDateAggregate::new(
      args.input_file(),
      |_| {},
   )?;

   // Create a data summary report
   if args.verbose() {println!(
      "Creating statistical report of the collected data...",
   )};
   let data = data.create_report();

   if let Some(file) = args.output_file() {
      if args.verbose() {println!(
         "Writing report to {file}...",
      )};
      std::fs::write(file, data.to_string())?;
   } else {
      print!("{data}");
   }

   // Return success
   return Ok(());
}

