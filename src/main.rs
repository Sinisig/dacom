fn main() -> Result<(), Box<dyn std::error::Error>> {
   // Parse command-line arguments
   let args = dacom::Args::new(std::env::args());
   
   // Collect dates from files
   let data = dacom::FileAggregateDateSet::new_recursive_with(
      &args.input_file(),
      |path| println!("Searched {}", path.to_str().unwrap()),
   )?;
   
   // Create a data report
   let data = data.create_report();

   // Send the data to the appropriate file stream
   if let Some(path) = &args.output_file() {
      std::fs::write(path, data.to_string())?;
   } else {
      print!("{data}");
   }

   // Return success
   return Ok(());
}

