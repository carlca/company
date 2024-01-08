use company::company::Company;
use futures::stream::StreamExt;
use mongodb::bson::{Bson, doc, Document};
use mongodb::{bson, Client};
use std::error::Error;
use std::io::Write;
use mongodb::options::FindOptions;

const TOTAL_COMPANY_COUNT: f64 = 5467419.0;

fn main() -> Result<(), Box<dyn Error>> {
  if let Err(e) = parse_company_data() {
    eprintln!("Error parsing company data: {:?}", e);
  }
  Ok(())
}

#[tokio::main]
async fn parse_company_data() -> Result<(), Box<dyn Error>> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let companies: mongodb::Collection<Document> = client.database("company").collection("company");
  let mut cursor = companies.find(None, None).await?;
  let mut count = 0;
  while let Some(result) = cursor.next().await {
    match result {
      Ok(doc) => {
        // The Bson document is successfully printed here
        if count == 1 {
          println!("{:?}", Bson::Document(doc.clone()));
          std::io::stdout().flush()?;
        }
        // This if fails presumably dues to some parse error
        let result = bson::from_bson(Bson::Document(doc));
        match result {
          Ok(company) => {
            let company: Company = company;
            let pretty_company = serde_json::to_string_pretty(&company).unwrap();
            if count == 1 {
              println!("{}", pretty_company);
              std::io::stdout().flush()?;
              break;
            }
          },
          Err(e) => eprintln!("Error: {}", e),
        }
      },
      Err(e) => eprintln!("Error: {}", e),
    }
    count += 1;
    if count % 1000 == 0 {
      print!("\rRecords processed: {} ({:.4}% done)", count, (count as f64 / TOTAL_COMPANY_COUNT) * 100.0);
      std::io::stdout().flush()?;
    }
  }
  print!("\rRecords processed: {} ({:.4}% done)", count, (count as f64 / TOTAL_COMPANY_COUNT) * 100.0);
  std::io::stdout().flush()?;

  let filter = doc! {
        "company_name": {
            "$regex": Bson::String("paramount syndications".to_string()),
            "$options": Bson::String("i".to_string())
        }
    };
  let find_options = FindOptions::builder().sort(doc! { "_id": -1 }).build();

  // Use find method with filter
  let mut cursor = companies.find(filter, find_options).await?;

  // Fetch the documents using while loop
  while let Some(result) = cursor.next().await {
    match result {
      Ok(document) => {
        if let Some(company_name) = document.get("company_name").and_then(Bson::as_str) {
          println!("{}", company_name);
        } else {
          println!("No Company Name found!");
        }
      }
      Err(e) => return Err(e.into()),
    }
  }

  Ok(())
}

