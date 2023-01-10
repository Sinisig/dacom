fn main() -> Result<(), Box<dyn std::error::Error>> {
   // Parse command-line arguments
   let args = dacom::Args::new(std::env::args());
   
   // Collect dates from files
   let data = dacom::FileAggregateDateSet::new_recursive_with(
      &args.input_file(),
      |path| println!("Searched {}", path.to_str().unwrap()),
   )?;
   
   // Print the results
   for file in data.iter() {
      println!("{}", file.path().to_str().unwrap());
      for date in file.dates().as_slice() {
         println!("   {date}");
      }
      println!("");
   }

   // Return success
   return Ok(());
}

