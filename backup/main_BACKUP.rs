mod company;

use company::Company;

use console::Term;
use csv::Reader;
use futures::stream::StreamExt;
use mongodb::bson::{doc, Bson, Document};
use mongodb::{bson, Client};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  if let Err(e) = read_first_23_docs() {
    eprintln!("Error reading first 23 MongoDB docs: {:?}", e);
  }
  // loop {
  //   let msg = "H to read csv headers,\n".to_owned()
  //     + "G to get MongoDB fields,\n"
  //     + "R to read first 23 MongoDB docs,\n"
  //     + "C to read first CR0 docs,\n"
  //     + "or Q to quit...";
  //   println!("{}", msg);
  //   let key = read_single_key();
  //   match key {
  //     Some('h') | Some('H') => {
  //       if let Err(e) = parse_csv_headers() {
  //         eprintln!("Error reading CSV headers: {:?}", e);
  //       }
  //     }
  //     Some('g') | Some('G') => {
  //       if let Err(e) = get_mongodb_fields() {
  //         eprintln!("Error setting up mongodb: {:?}", e);
  //       }
  //     }
  //     Some('r') | Some('R') => {
  //       if let Err(e) = read_first_23_docs() {
  //         eprintln!("Error reading first 23 MongoDB docs: {:?}", e);
  //       }
  //     }
  //     Some('c') | Some('C') => {
  //       if let Err(e) = read_cr0_docs() {
  //         eprintln!("Error reading CR0 docs: {:?}", e);
  //       }
  //     }
  //     Some('q') | Some('Q') => break,
  //     _ => (),
  //   }
  // }
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
      println!(
        "Field names and types for collection '{}':",
        collection.name()
      );
      print_fields(&document, String::new());
    }
    None => println!("No document found"),
  }

  Ok(())
}

#[tokio::main]
async fn read_first_23_docs() -> Result<(), Box<dyn Error>> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let companies: mongodb::Collection<Document> = client.database("local").collection("company");
  let mut cursor = companies.find(None, None).await?;
  let mut count = 0;
  while let Some(result) = cursor.next().await {
    match result {
      Ok(doc) => {
        // The Bson document is successfully printer here
        println!("{:?}", Bson::Document(doc.clone()));
        // This if fails presumably dues to some parse error
        let result = bson::from_bson(Bson::Document(doc));
        match result {
          Ok(company) => {
            let company: Company = company;
            let pretty_company = serde_json::to_string_pretty(&company).unwrap();
            println!("{}", pretty_company);
          },
          Err(e) => eprintln!("Error: {}", e),
        }
      },
      Err(e) => eprintln!("Error: {}", e),
    }
    count += 1;
    if count == 23 {
      break;
    }
  }
  Ok(())
}

#[tokio::main]
async fn read_cr0_docs() -> Result<(), Box<dyn Error>> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let mut cursor = client
    .database("local")
    .collection::<Document>("company")
    .find(
      doc! {
        "RegAddress.PostCode": doc! {
            "$regex": "CR0"
        }
      },
      None,
    )
    .await?;
  while let Some(result) = cursor.next().await {
    let comp: Document = result?;
    print!("Company: {}", comp.get_str("CompanyName")?,);
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
      }
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
  }
  .to_string()
}
