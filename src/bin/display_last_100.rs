use std::fs::File;
use std::process;
use std::error::Error;
use std::io::Write;

const TOTAL_COMPANY_COUNT: f64 = 5467419.0;

fn main() {
  if let Err(err) = run() {
    println!("{}", err);
    process::exit(1);
  }
}

fn run() -> Result<(), Box<dyn Error>> {
  let csv_file = File::open("./data/Company2.csv")?;
  // create reader for input csv file
  let mut reader = csv::Reader::from_reader(csv_file);
  let mut count: i32 = 0;
  for result in reader.records() {
    let record = result?;
    count += 1;
    if count % 1000 == 0 {
      print!("\rRecords processed: {} ({:.4}% done)", count, (count as f64 / TOTAL_COMPANY_COUNT) * 100.0);
      std::io::stdout().flush()?;
    }
    if count as f64 >= TOTAL_COMPANY_COUNT - 100.0 {
      println!("{:?}", record);
    }
  }
  Ok(())
}

