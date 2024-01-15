use std::env;
use std::fs::File;
use std::process;
use std::error::Error;
use std::io::Write;
use csv::StringRecord;
use company::field_map::FieldMap;

const TOTAL_COMPANY_COUNT: f64 = 5476771.0;

fn main() {
  let args: Vec<String> = env::args().collect();
  let cmd = args[0].to_string();
  let mut data_folder = "../data";
  if is_dev(&cmd) {
    data_folder = "./data";
  }
  if let Err(err) = run(data_folder) {
    println!("{}", err);
    process::exit(1);
  }
}

fn is_dev(cmd: &String) -> bool {
  cmd.contains("target")
}

fn run(data_folder: &str) -> Result<(), Box<dyn Error>> {
  // open input csv file
  let input_file = File::open(data_folder.to_owned() + "/company_old.csv")?;
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
  let mut writer = csv::Writer::from_path(data_folder.to_owned() + "/company.csv")?;
  let _ = writer.write_record(&StringRecord::from(output_headers));
  for result in reader.records() {
    let record = result?;
    writer.write_record(&record)?;
    count += 1;
    if count % 1000 == 0 {
      let _ = show_progress(&mut count, false);
    }
  }
  let _ = show_progress(&mut count, true);
  let _ = writer.flush();

  Ok(())
}

fn show_progress(count: &mut i32, show_all: bool) -> Result<(), Box<dyn Error>> {
  if show_all || *count % 1000 == 0 {
    print!("\rRecords processed: {} ({:.0}% done)", count, (*count as f64 / TOTAL_COMPANY_COUNT) * 100.0);
    std::io::stdout().flush()?;
  }
  Ok(())
}
