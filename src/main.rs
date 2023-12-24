use std::error::Error;
use csv::Reader;
// use sled::{Config, Db};
// use std::io::Write;
use console::Term;

// type StdResult<T, E> = std::result::Result<T, E>;
// type MongoResult<T> = mongodb::error::Result<T>;

fn main() -> Result<(), Box<dyn Error>> {
  loop {
    let msg = "H to read csv headers,\n".to_owned() +
              "P to parse csv data,\n" +
              "R to read sled data,\n" +
              "M to setup MongoDB,\n" +
              "or Q to quit...";
    println!("{}", msg);
    let key = read_single_key();
    match key {
      Some('h') | Some('H') => {
        if let Err(e) = parse_csv_headers() {
          eprintln!("Error reading CSV headers: {:?}", e);
        }
      }
      // Some('p') | Some('P') => {
      //   if let Err(e) = parse_csv_data() {
      //     eprintln!("Error parsing CSV data: {:?}", e);
      //   }
      // }
      // Some('r') | Some('R') => {
      //   if let Err(e) = read_sled_data() {
      //     eprintln!("Error reading sled data: {:?}", e);
      //   }
      // }
      // Some('m') | Some('M') => {
      //   if let Err(e) = setup_mongodb() {
      //     eprintln!("Error setting up mongodb: {:?}", e);
      //   }
      // }
      Some('q') | Some('Q') => break,
      _ => (),
    }
  }
  Ok(())
}

fn parse_csv_headers() -> Result<(), Box<dyn Error>> {
  let mut rdr = Reader::from_path("./data/Company.csv")?;

  let headers = rdr.headers()?.clone();
  for header in &headers {
    println!("{:?}", header)
  }

  Ok(())
}

// #[tokio::main]
// async fn parse_csv_data() -> Result<()> {
//   let db = Config::new().path("my_db").open()?;
//   let mut rdr = Reader::from_path("./data/Company.csv")?;
//
//   // Get the headers
//   let headers = rdr.headers()?.clone();
//
//   // Specify the field names for which keys should be created
//   let key_field_names = ["CompanyName", "RegAddress.PostTown", "RegAddress.PostCode"];
//
//   // Print the headers
//   println!("{:?}", headers);
//
//   let mut count = 0;
//   for result in rdr.records() {
//     if let Ok(record) = result {
//       let primary_key = &record[0];
//       for (i, value) in record.iter().enumerate().skip(1) {
//         let header_name = headers.get(i).unwrap_or(&"");
//
//         // Check if the header name is one of the specified keys
//         if key_field_names.contains(&header_name) {
//           let key = format!("{}_{}", primary_key, header_name);
//           db.insert(key.as_bytes(), value.as_bytes())?;
//         }
//       }
//       count += 1;
//       print!("\rRecords processed: {} ({:.4}% done)", count, (count as f64 / 5467419.0) * 100.0);
//       std::io::stdout().flush()?; // makes sure the printed text appears immediately
//     }
//   }
//
//   println!(); // final newline for clean terminal prompt
//   db.flush()?;
//
//   Ok(())
// }

// #[tokio::main]
// async fn read_sled_data() -> Result<()> {
//   let db: Db = Config::new().path("my_db").open()?;
//
//   let iter = db.iter();
//   for item in iter {
//     let (key, value) = item?;
//     println!("Key: {}, Value: {}",
//              std::str::from_utf8(&key).unwrap_or("Invalid UTF-8 sequence"),
//              std::str::from_utf8(&value).unwrap_or("Invalid UTF-8 sequence"),
//     );
//   }
//   Ok(())
// }

// #[tokio::main]
// async fn setup_mongodb() -> StdResult<(), mongodb::error::Error> {
//   let client = Client::with_uri_str("mongodb://localhost:27017").await?;
//   let databases = client.list_database_names(None, None).await?;
//
//   // Print out the names of the databases
//   for db_name in databases {
//     println!("{}", db_name);
//   }
//   Ok(())
// }

fn read_single_key() -> Option<char> {
  let term = Term::buffered_stdout();
  term.read_char().ok()
}