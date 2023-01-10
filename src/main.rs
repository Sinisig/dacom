fn main() -> Result<(), Box<dyn std::error::Error>> {
   // Parse command-line arguments
   let args = dacom::Args::new(std::env::args());
   
   // Load dates from files
   let data = dacom::FileDateAggregate::new(
      &args.input_file(),
      |file| println!("Searching {file}"),
   )?;

   // Test code for new stuff
   println!("{}", data.create_report());

   // Return success
   return Ok(());
}

