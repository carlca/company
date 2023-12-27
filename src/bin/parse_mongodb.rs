use company::company::Company;
use futures::stream::StreamExt;
use mongodb::bson::{Bson, Document};
use mongodb::{bson, Client};
use std::error::Error;
use std::io::Write;

const TOTAL_COMPANY_COUNT: f64 = 5467419.0;

fn main() -> Result<(), Box<dyn Error>> {
  if let Err(e) = read_n_docs() {
    eprintln!("Error reading first 23 MongoDB docs: {:?}", e);
  }
  Ok(())
}

#[tokio::main]
async fn read_n_docs() -> Result<(), Box<dyn Error>> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let companies: mongodb::Collection<Document> = client.database("local").collection("company");
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
            }
          },
          Err(e) => eprintln!("Error: {}", e),
        }
      },
      Err(e) => eprintln!("Error: {}", e),
    }
    count += 1;
    if count as f64 == TOTAL_COMPANY_COUNT {
      break;
    }
    print!("\rRecords processed: {} ({:.4}% done)", count, (count as f64 / TOTAL_COMPANY_COUNT) * 100.0);
    std::io::stdout().flush()?;
  }
  Ok(())
}

