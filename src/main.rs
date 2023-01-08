fn date_file(file : std::fs::File) -> Vec<dacom::Date> {
   return Vec::new();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   for arg in std::env::args() {
      println!("{arg}");
   }

   return Ok(());
}

