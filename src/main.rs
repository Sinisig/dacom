fn main() -> Result<(), Box<dyn std::error::Error>> {
   // Parse command-line arguments
   let args = dacom::Args::new(std::env::args());
   
   // Load dates from files
   let data = dacom::FileDateAggregate::new(
      &args.input_file(),
      |path| println!("Searching {path}"),
   )?;

   // Test code for new stuff
   data.iter().for_each(|data| {
      println!("{}", data.path());
      data.dates().iter().for_each(|date| {
         println!("   {}", date);
      });
      println!("");
   });

   // Return success
   return Ok(());
}

