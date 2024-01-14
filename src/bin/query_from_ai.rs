extern crate maplit;
extern crate serde_json;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;

macro_rules! query {
  ($( $key:expr => $fn:expr ),*) => {
    {
      let mut map = HashMap::new();
      $(
        map.insert($key.to_string(), $fn);
      )*
      map
    }
  };
}

macro_rules! op {
  ($op:expr, $val:expr) => {
    {
      let mut map = HashMap::new();
      map.insert($op.to_string(), json!($val));
      json!(map)
    }
  };
}

macro_rules! val {
  ($val:expr) => {
    {
      json!($val)
    }
  };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  match run_query().await {
    Ok(result) => {
      println!("Success: {:?}", result);
      Ok(())
    }
    Err(e) => {
      eprintln!("Error running query: {:?}", e);
      Err(e)
    }
  }
}

async fn run_query() -> Result<(), Box<dyn Error>> {
  let mongo_query = query! {
    "age" => op!("$gt", 30), "name" => val!("John Doe")
  };
  println!("{:?}", mongo_query);

  // add your async database operations here...
  Ok(())
}