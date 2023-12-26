use mongodb::bson::oid::ObjectId;
use serde::de::{Deserializer, Error, Unexpected, Visitor};
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum NumberOrString {
  Number(i32),
  String(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Company {
  #[serde(rename = "_id")]
  id: ObjectId,
  #[serde(rename = "CompanyName")]
  company_name: Option<String>,
  #[serde(rename = "CompanyNumber", deserialize_with = "number_or_string")]
  company_number: NumberOrString,
  #[serde(rename = "RegAddress")]
  reg_address: RegAddress,
  #[serde(rename = "CompanyCategory")]
  company_category: Option<String>,
  #[serde(rename = "CompanyStatus")]
  company_status: Option<String>,
  #[serde(rename = "CountryOfOrigin")]
  country_of_origin: Option<String>,
  #[serde(rename = "DissolutionDate")]
  dissolution_date: Option<String>,
  #[serde(rename = "IncorporationDate")]
  incorporation_date: Option<String>,
  #[serde(rename = "Accounts")]
  accounts: Accounts,
  #[serde(rename = "Returns")]
  returns: Returns,
  #[serde(rename = "Mortgages")]
  mortgages: Mortgages,
  #[serde(rename = "SICCode")]
  sic_code: SICCode,
  #[serde(rename = "LimitedPartnerships")]
  limited_partnerships: LimitedPartnerships,
  #[serde(rename = "URI")]
  uri: Option<String>,
  #[serde(rename = "PreviousName_1")]
  previous_name_1: Option<PreviousName>,
  #[serde(rename = "PreviousName_2")]
  previous_name_2: Option<PreviousName>,
  #[serde(rename = "PreviousName_3")]
  previous_name_3: Option<PreviousName>,
  #[serde(rename = "PreviousName_4")]
  previous_name_4: Option<PreviousName>,
  #[serde(rename = "PreviousName_5")]
  previous_name_5: Option<PreviousName>,
  #[serde(rename = "PreviousName_6")]
  previous_name_6: Option<PreviousName>,
  #[serde(rename = "PreviousName_7")]
  previous_name_7: Option<PreviousName>,
  #[serde(rename = "PreviousName_8")]
  previous_name_8: Option<PreviousName>,
  #[serde(rename = "PreviousName_9")]
  previous_name_9: Option<PreviousName>,
  #[serde(rename = "PreviousName_10")]
  previous_name_10: Option<PreviousName>,
  #[serde(rename = "ConfStmtNextDueDate")]
  conf_stmt_next_due_date: Option<String>,
  #[serde(rename = "confStmtLastMadeUpDate")]
  conf_stmt_last_made_up_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegAddress {
  #[serde(rename = "CareOf")]
  care_of: Option<String>,
  #[serde(rename = "POBox")]
  po_box: Option<String>,
  #[serde(rename = "AddressLine1")]
  address_line_1: Option<String>,
  #[serde(rename = "AddressLine2")]
  address_line_2: Option<String>,
  #[serde(rename = "PostTown")]
  post_town: Option<String>,
  #[serde(rename = "County")]
  county: Option<String>,
  #[serde(rename = "Country")]
  country: Option<String>,
  #[serde(rename = "PostCode")]
  post_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Accounts {
  #[serde(rename = "AccountRefDay")]
  account_ref_day: i32,
  #[serde(rename = "AccountRefMonth")]
  account_ref_month: i32,
  #[serde(rename = "NextDueDate")]
  next_due_date: Option<String>,
  #[serde(rename = "LastMadeUpDate")]
  last_made_up_date: Option<String>,
  #[serde(rename = "AccountCategory")]
  account_category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
  #[serde(rename = "NextDueDate")]
  next_due_date: Option<String>,
  #[serde(rename = "LastMadeUpDate")]
  last_made_up_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mortgages {
  #[serde(rename = "NumMortCharges")]
  num_mort_charges: i32,
  #[serde(rename = "NumMortOutstanding")]
  num_mort_outstanding: i32,
  #[serde(rename = "NumMortPartSatisfied")]
  num_mort_part_satisfied: i32,
  #[serde(rename = "NumMortSatisfied")]
  num_mort_satisfied: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SICCode {
  #[serde(rename = "SicText_1")]
  sic_text_1: Option<String>,
  #[serde(rename = "SicText_2")]
  sic_text_2: Option<String>,
  #[serde(rename = "SicText_3")]
  sic_text_3: Option<String>,
  #[serde(rename = "SicText_4")]
  sic_text_4: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LimitedPartnerships {
  #[serde(rename = "NumGenPartners")]
  num_gen_partners: i32,
  #[serde(rename = "NumLimPartners")]
  num_lim_partners: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousName {
  #[serde(rename = "CONDATE")]
  con_date: Option<String>,
  #[serde(rename = "CompanyName")]
  company_name: Option<String>,
}

fn number_or_string<'de, D>(deserializer: D) -> Result<NumberOrString, D::Error>
  where
    D: Deserializer<'de>,
{
  struct NumberOrStringVisitor;

  impl<'de> Visitor<'de> for NumberOrStringVisitor {
    type Value = NumberOrString;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("a string or a number")
    }

    fn visit_i64<E>(self, v: i64) -> Result<NumberOrString, E>
      where
        E: Error,
    {
      Ok(NumberOrString::Number(v as i32))
    }

    fn visit_u64<E>(self, v: u64) -> Result<NumberOrString, E>
      where
        E: Error,
    {
      Ok(NumberOrString::Number(v as i32))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: Error,
    {
      match v.parse::<i32>() {
        Ok(i) => Ok(NumberOrString::Number(i)),
        Err(_) => Ok(NumberOrString::String(v.to_string()))
      }
    }
  }
  deserializer.deserialize_any(NumberOrStringVisitor)
}