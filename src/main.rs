fn main() -> Result<(), Box<dyn std::error::Error>> {
   let args = dacom::Args::new(std::env::args());

   args.input_files().iter().for_each(|f| {
   println!("Input file:  {f}");
   });
   println!("Output file: {}", args.output_file());
   println!("Verbose:     {}", args.verbose());

   return Ok(());
}

