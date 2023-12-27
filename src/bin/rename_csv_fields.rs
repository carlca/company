use std::fs::File;
use std::process;
use std::error::Error;
use company::field_map::FieldMap;

fn main() {
  if let Err(err) = run() {
    println!("{}", err);
    process::exit(1);
  }
}

fn run() -> Result<(), Box<dyn Error>> {
  // open input csv file
  let input_file = File::open("./data/Company.csv")?;
  // create reader for input csv file
  let mut reader = csv::Reader::from_reader(input_file);
  // create FieldMap
  let map = FieldMap::new();

  let headers = reader.headers()?.clone();
  let output_headers: Vec<String> = headers
    .iter()
    .map(|field| {
      // Convert field to String since it is a &str
      let field = field.to_string();
      // If the field is found in the FieldMap, it will be replaced by the new name
      // else, the original name will be retained.
      map.get(&field)
    })
    .collect();

  for header in output_headers {
    println!("{}", header);
  }

  Ok(())
}

