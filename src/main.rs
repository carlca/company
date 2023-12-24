use std::error::Error;
use csv::Reader;
use console::Term;
use mongodb::{Client};
use mongodb::bson::{Bson, Document};
use futures::stream::StreamExt;

fn main() -> Result<(), Box<dyn Error>> {
  loop {
    let msg = "H to read csv headers,\n".to_owned() +
              "G to get MongoDB fields,\n" +
              "R to read first 23 MongoDB docs,\n" +
              "or Q to quit...";
    println!("{}", msg);
    let key = read_single_key();
    match key {
      Some('h') | Some('H') => {
        if let Err(e) = parse_csv_headers() {
          eprintln!("Error reading CSV headers: {:?}", e);
        }
      }
      Some('g') | Some('G') => {
        if let Err(e) = get_mongodb_fields() {
          eprintln!("Error setting up mongodb: {:?}", e);
        }
      }
      Some('r') | Some('R') => {
        if let Err(e) = read_first_23_docs() {
          eprintln!("Error reading first 23 MongoDB docs: {:?}", e);
        }
      }
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

#[tokio::main]
async fn get_mongodb_fields() -> Result<(), mongodb::error::Error> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let databases = client.list_database_names(None, None).await?;

  // Print out the names of the databases
  for db in databases {
    println!("{}", db);
  }

  let db = client.database("local");
  let collection = db.collection::<Document>("company");

  match collection.find_one(None, None).await? {
    Some(document) => {
      println!("Field names and types for collection '{}':", collection.name());
      print_fields(&document, String::new());
    },
    None => println!("No document found"),
  }

  Ok(())
}

#[tokio::main]
async fn read_first_23_docs() -> Result<(), Box<dyn Error>> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let companies = client.database("local").collection("company");
  let mut cursor = companies.find(None, None).await?;
  let mut count = 0;
  while let Some(result) = cursor.next().await {
    let comp: Document = result?;
    println!("Company: {}", comp.get_str("CompanyName")?);
    count += 1;
    if count == 23 {
      break;
    }
  }

  Ok(())
}

fn print_fields(document: &Document, indent: String) {
  for (key, value) in document.iter() {
    match value {
      Bson::Document(nested_doc) => {
        println!("{}Field name: '{}', Type: 'Document'", indent, key);
        // Recurse into the nested document with increased indentation
        print_fields(nested_doc, format!("{}  ", indent));
      },
      _ => {
        let field_type = match_type(&value);
        println!("{}Field name: '{}', Type: '{}'", indent, key, field_type);
      }
    }
  }
}

fn read_single_key() -> Option<char> {
  let term = Term::buffered_stdout();
  term.read_char().ok()
}

fn match_type(b: &Bson) -> String {
  match b {
    Bson::Double(_) => "Double",
    Bson::String(_) => "String",
    Bson::Array(_) => "Array",
    Bson::Document(_) => "Document",
    Bson::Boolean(_) => "Boolean",
    Bson::Null => "Null",
    Bson::RegularExpression(_) => "RegularExpression",
    Bson::JavaScriptCode(_) => "JavaScriptCode",
    Bson::ObjectId(_) => "ObjectId",
    Bson::DateTime(_) => "DateTime",
    Bson::Symbol(_) => "Symbol",
    Bson::JavaScriptCodeWithScope(_) => "JavaScriptCodeWithScope",
    Bson::Timestamp { .. } => "Timestamp",
    Bson::Binary(_) => "Binary",
    Bson::Decimal128(_) => "Decimal128",
    Bson::MaxKey => "MaxKey",
    Bson::MinKey => "MinKey",
    Bson::Undefined => "Undefined",
    Bson::Int32(_) => "Int32",
    Bson::Int64(_) => "Int64",
    _ => "",
  }.to_string()
}