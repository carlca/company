use std::fs::File;
use std::process;
use std::error::Error;
use std::io::Write;
use csv::StringRecord;
use company::field_map::FieldMap;

const TOTAL_COMPANY_COUNT: f64 = 5476771.0;

fn main() {
  if let Err(err) = run() {
    println!("{}", err);
    process::exit(1);
  }
}

fn run() -> Result<(), Box<dyn Error>> {
  // open input csv file
  let input_file = File::open("../data/company_old.csv")?;
  // create reader for input csv file
  let mut reader = csv::Reader::from_reader(input_file);
  // create FieldMap
  let map = FieldMap::new();

  let headers = reader.headers()?.clone();
  let output_headers: Vec<String> = headers
    .iter()
    .map(|field| {
      let binding = field.to_string();
      let field = binding.trim_start();
      map.get(&field)
    })
    .collect();

  let mut count = 0;
  let mut writer = csv::Writer::from_path("../data/company.csv")?;
  let _ = writer.write_record(&StringRecord::from(output_headers));
  for result in reader.records() {
    let record = result?;
    writer.write_record(&record)?;
    count += 1;
    if count % 1000 == 0 {
      show_progress(&mut count, false);
    }
  }
  show_progress(&mut count, true);
  let _ = writer.flush();

  Ok(())
}

fn show_progress(count: &mut i32, show_all: bool) -> Result<(), Box<dyn Error>> {
  if show_all || *count % 1000 == 0 {
    print!("\rRecords processed: {} ({:.4}% done)", count, (*count as f64 / TOTAL_COMPANY_COUNT) * 100.0);
    std::io::stdout().flush()?;
  }
  Ok(())
}
