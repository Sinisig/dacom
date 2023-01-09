fn main() -> Result<(), Box<dyn std::error::Error>> {
   // Parse command-line arguments
   let args = std::sync::Arc::new(std::sync::Mutex::new(
      dacom::Args::new(std::env::args()),
   ));
   
   // Create channel for receiving date analysis results
   if args.lock().unwrap().verbose() {println!(
      "Creating data pipes",
   )};
   let (
      pipe_results_transmitter,
      pipe_results_receiver,
   ) = std::sync::mpsc::channel::<Vec<(String, Vec<dacom::Date>)>>();
   
   // Create threads to harvest date information from every input file
   if args.lock().unwrap().verbose() {println!(
      "Starting date collector threads",
   )};
   for path in args.lock().unwrap().input_files() {
      let args = args.clone();
      let path = path.clone();
      let tx   = pipe_results_transmitter.clone();
      std::thread::spawn(move || {
         match file_collect_dates(path, &args.lock().unwrap()) {
            Ok(dates)   => tx.send(dates).unwrap(),
            Err(err)    => eprintln!("File I/O error: {err}"),
         }
      });
   }
   std::mem::drop(pipe_results_transmitter);

   // Combine all the results together
   if args.lock().unwrap().verbose() {println!(
      "Waiting for date collection results"
   )};
   let mut results = Vec::new();
   while let Ok(mut r) = pipe_results_receiver.recv() {
      results.append(&mut r);
   }

   // Sort the results by oldest date, then newest date, then path
   if args.lock().unwrap().verbose() {println!(
      "Sorting results",
   )};
   results.sort_unstable_by(|d1, d2| {
      use std::cmp::Ordering::*;
      match d1.1.first().unwrap().cmp(&d2.1.first().unwrap()) {
         Greater  => Greater,
         Less     => Less,
         Equal    => match d1.1.last().unwrap().cmp(&d2.1.last().unwrap()) {
            Greater  => Greater,
            Less     => Less,
            Equal    => d1.0.cmp(&d2.0),
         }
      }
   });

   // Stringify the compiled list of dates
   if args.lock().unwrap().verbose() {println!(
      "Stringifying results",
   )};
   let mut results_text = String::new();
   for (path, dates) in results.into_iter() {
      results_text += &format!("{path}\n");
      for date in dates {
         results_text += &format!("   {date}\n");
      }
      results_text += "\n";
   }
   
   // Write the stringified results to the output file
   if args.lock().unwrap().verbose() {println!(
      "Writing results to {}",
      args.lock().unwrap().output_file(),
   )};
   std::fs::write(args.lock().unwrap().output_file(), &results_text)?;

   // Return success
   return Ok(());
}

// Searches a file for dates and compiles them
fn file_collect_dates(
   path  : String,
   args  : & dacom::Args,
) -> Result<Vec<(String, Vec<dacom::Date>)>, std::io::Error> {
   if args.verbose() {println!(
      "Searching {path}",
   )}

   let mut dates = Vec::new();

   // If it's a directory, recursively date its contents
   // Otherwise run the dater on it
   if std::fs::File::open(&path)?.metadata()?.is_dir() {
      for file in std::fs::read_dir(&path)? {
         let file = file?;
         let path = String::from(file.path().to_str().unwrap());
         let mut dates_add = file_collect_dates(path, args)?;
         dates.append(&mut dates_add);
      }
   } else {
      // Load the file's contents into memory
      let file = std::fs::read(&path)?;
      let file = match String::from_utf8(file) {
         Ok(f)    => f,
         Err(_)   => return Ok(dates),
      };

      // Search the text for dates and sort them into the vector
      let date_list = dacom::Date::from_text_multi_sorted_by(
         &file,
         |&d1, &d2| d1.cmp(&d2),
      );

      // Add the date list only if we found anything
      if date_list.is_empty() == false {
         dates.push((path, date_list));
      }
   }

   return Ok(dates);
}

