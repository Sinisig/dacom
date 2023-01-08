// Dates every file in a directory and sorts
// the contents from oldest to newest.
fn date_directory_sorted(
   path : & str,
) -> Result<Vec<(String, Vec<dacom::Date>)>, Box<dyn std::error::Error>> {
   let mut dates = Vec::new();

   // Iterate for every element in the current directory
   for ent in std::fs::read_dir(path)? {
      let ent  = ent?;
      let path = ent.path();
      let path = path.to_str().unwrap();

      if ent.file_type()?.is_dir() == true {
         // Recursively search subdirectories
         let mut sub_info = date_directory_sorted(path)?;
         dates.append(&mut sub_info);

      } else {
         // Read the file text
         let text = std::fs::read(&path)?;
         let text = String::from_utf8_lossy(&text);

         // Search for all dates in the text and sort them
         let mut found_dates = dacom::Date::from_string(&text);
         found_dates.sort_unstable();

         // Add the result to the running total if we found any
         if found_dates.len() != 0 {
            dates.push((String::from(path), found_dates));
         }
      }
   }

   // Sort all the files in order using the oldest date
   dates.sort_unstable_by(|a, b| {
      use std::cmp::Ordering::*;

      // Only care about dating info
      let a = &a.1;
      let b = &b.1;

      // First try matching by oldest file date
      return match a.first().unwrap().cmp(b.first().unwrap()) {
         Less     => Less,
         Greater  => Greater,
         Equal    => {
            // Oldest file dates match, try comparing
            // newest file date
            match a.last().unwrap().cmp(b.last().unwrap()) {
               Less     => Less,
               Greater  => Greater,
               Equal    => {
                  // Give up.  Just use counts
                  a.len().cmp(&b.len())
               },
            }
         },
      };
   });

   return Ok(dates);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let mut args = std::env::args();
   args.next();   // Skips the executable path

   for arg in args {
      println!("Seaching directory {arg}");
      let search_results = date_directory_sorted(&arg)?;

      println!("Done! Here are the results:");
      println!("");
      for (file, dates) in &search_results {
         println!("{file}");
         for date in dates {
            println!("   {date}");   
         }
         println!("");
      }

      println!("");
   }

   return Ok(());
}

