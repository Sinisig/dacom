fn main() -> Result<(), Box<dyn std::error::Error>> {
   // Parse command-line arguments
   let args = dacom::Args::new(std::env::args());
   
   // Load dates from files
   let data = dacom::FileDateAggregate::new(
      &args.input_files()[0],
      if args.verbose() {
         |path| println!("Searching {path}")
      } else {
         |_| {}
      },
   )?;

   // Test code for new stuff
   data.iter().for_each(|data| {
      println!("{}", data.path());
      data.dates().iter().for_each(|date| {
         println!("   {}", date);
      });
   });

   // Return success
   return Ok(());
}

