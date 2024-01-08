use std::fs::File;
use std::process;
use std::error::Error;
use std::io::Write;
use csv::StringRecord;
use company::field_map::FieldMap;

const TOTAL_COMPANY_COUNT: f64 = 5467419.0;

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
      let field = field.to_string();
      map.get(&field)
    })
    .collect();

  let mut count = 0;
  let mut writer = csv::Writer::from_path("./data/Company2.csv")?;
  let _ = writer.write_record(&StringRecord::from(output_headers));
  for result in reader.records() {
    let record = result?;
    writer.write_record(&record)?;
    count += 1;
    if count % 1000 == 0 {
      show_progress(&count);
    }
  }
  show_progress(&count);
  let _ = writer.flush();

  Ok(())
}

fn show_progress(count: &i32) {
  print!("\rRecords processed: {} ({:.4}% done)", count, (*count as f64 / TOTAL_COMPANY_COUNT) * 100.0);
  std::io::stdout().flush().unwrap();
}
