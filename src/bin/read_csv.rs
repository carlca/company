use std::{
  error::Error,
  fs::File,
  process,
};

fn run() -> Result<(), Box<dyn Error>> {
  let input_file = File::open("./data/company_old.csv")?;
  let mut reader = csv::Reader::from_reader(input_file);
  {
    // We nest this call in its own scope because of lifetimes.
    let headers = reader.headers()?;
    println!("{:?}", headers);
  }
  //
  // let mut rdr = csv::Reader::from_reader(file);
  // for result in rdr.records() {
  //   let record = result?;
  //   println!("{:?}", record);
  //   break;
  // }
  Ok(())
}

fn main() {
  if let Err(err) = run() {
    println!("{}", err);
    process::exit(1);
  }
}