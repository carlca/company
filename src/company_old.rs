use mongodb::bson::Bson;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Variant {
  Vec(Bson),
  Number(i32),
  Float(f64),
  String(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Company {
  #[serde(rename = "_id")]
  id: Option<ObjectId>,
  #[serde(rename = "CompanyName")]
  company_name: Option<Variant>,
  #[serde(rename = "CompanyNumber")]
  company_number: Option<Variant>,
  #[serde(rename = "RegAddress")]
  reg_address: Option<RegAddress>,
  #[serde(rename = "CompanyCategory")]
  company_category: Option<Variant>,
  #[serde(rename = "CompanyStatus")]
  company_status: Option<Variant>,
  #[serde(rename = "CountryOfOrigin")]
  country_of_origin: Option<Variant>,
  #[serde(rename = "DissolutionDate")]
  dissolution_date: Option<Variant>,
  #[serde(rename = "IncorporationDate")]
  incorporation_date: Option<Variant>,
  #[serde(rename = "Accounts")]
  accounts: Option<Accounts>,
  #[serde(rename = "Returns")]
  returns: Option<Returns>,
  #[serde(rename = "Mortgages")]
  mortgages: Option<Mortgages>,
  #[serde(rename = "SICCode")]
  sic_code: Option<SICCode>,
  #[serde(rename = "LimitedPartnerships")]
  limited_partnerships: Option<LimitedPartnerships>,
  #[serde(rename = "URI")]
  uri: Option<Variant>,
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
  conf_stmt_next_due_date: Option<Variant>,
  #[serde(rename = "confStmtLastMadeUpDate")]
  conf_stmt_last_made_up_date: Option<Variant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegAddress {
  #[serde(rename = "CareOf")]
  care_of: Option<Variant>,
  #[serde(rename = "POBox")]
  po_box: Option<Variant>,
  #[serde(rename = "AddressLine1")]
  address_line_1: Option<Variant>,
  #[serde(rename = "AddressLine2")]
  address_line_2: Option<Variant>,
  #[serde(rename = "PostTown")]
  post_town: Option<Variant>,
  #[serde(rename = "County")]
  county: Option<Variant>,
  #[serde(rename = "Country")]
  country: Option<Variant>,
  #[serde(rename = "PostCode")]
  post_code: Option<Variant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Accounts {
  #[serde(rename = "AccountRefDay")]
  account_ref_day: Option<i32>,
  #[serde(rename = "AccountRefMonth")]
  account_ref_month: Option<i32>,
  #[serde(rename = "NextDueDate")]
  next_due_date: Option<Variant>,
  #[serde(rename = "LastMadeUpDate")]
  last_made_up_date: Option<Variant>,
  #[serde(rename = "AccountCategory")]
  account_category: Option<Variant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
  #[serde(rename = "NextDueDate")]
  next_due_date: Option<Variant>,
  #[serde(rename = "LastMadeUpDate")]
  last_made_up_date: Option<Variant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mortgages {
  #[serde(rename = "NumMortCharges")]
  num_mort_charges: Option<i32>,
  #[serde(rename = "NumMortOutstanding")]
  num_mort_outstanding: Option<i32>,
  #[serde(rename = "NumMortPartSatisfied")]
  num_mort_part_satisfied: Option<i32>,
  #[serde(rename = "NumMortSatisfied")]
  num_mort_satisfied: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SICCode {
  #[serde(rename = "SicText_1")]
  sic_text_1: Option<Variant>,
  #[serde(rename = "SicText_2")]
  sic_text_2: Option<Variant>,
  #[serde(rename = "SicText_3")]
  sic_text_3: Option<Variant>,
  #[serde(rename = "SicText_4")]
  sic_text_4: Option<Variant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LimitedPartnerships {
  #[serde(rename = "NumGenPartners")]
  num_gen_partners: Option<i32>,
  #[serde(rename = "NumLimPartners")]
  num_lim_partners: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousName {
  #[serde(rename = "CONDATE")]
  con_date: Option<Variant>,
  #[serde(rename = "CompanyName")]
  company_name: Option<Variant>,
}
