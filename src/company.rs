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
  company_name: Option<Variant>,
  company_number: Option<Variant>,
  reg_address: Option<RegAddress>,
  company_category: Option<Variant>,
  company_status: Option<Variant>,
  country_of_origin: Option<Variant>,
  dissolution_date: Option<Variant>,
  incorporation_date: Option<Variant>,
  accounts: Option<Accounts>,
  returns: Option<Returns>,
  mortgages: Option<Mortgages>,
  sic_code: Option<SICCode>,
  limited_partnerships: Option<LimitedPartnerships>,
  uri: Option<Variant>,
  previous_name_1: Option<PreviousName>,
  previous_name_2: Option<PreviousName>,
  previous_name_3: Option<PreviousName>,
  previous_name_4: Option<PreviousName>,
  previous_name_5: Option<PreviousName>,
  previous_name_6: Option<PreviousName>,
  previous_name_7: Option<PreviousName>,
  previous_name_8: Option<PreviousName>,
  previous_name_9: Option<PreviousName>,
  previous_name_10: Option<PreviousName>,
  conf_stmt_next_due_date: Option<Variant>,
  conf_stmt_last_made_up_date: Option<Variant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegAddress {
  care_of: Option<Variant>,
  po_box: Option<Variant>,
  address_line_1: Option<Variant>,
  address_line_2: Option<Variant>,
  post_town: Option<Variant>,
  county: Option<Variant>,
  country: Option<Variant>,
  post_code: Option<Variant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Accounts {
  account_ref_day: Option<i32>,
  account_ref_month: Option<i32>,
  next_due_date: Option<Variant>,
  last_made_up_date: Option<Variant>,
  account_category: Option<Variant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
  next_due_date: Option<Variant>,
  last_made_up_date: Option<Variant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mortgages {
  num_mort_charges: Option<i32>,
  num_mort_outstanding: Option<i32>,
  num_mort_part_satisfied: Option<i32>,
  num_mort_satisfied: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SICCode {
  sic_text_1: Option<Variant>,
  sic_text_2: Option<Variant>,
  sic_text_3: Option<Variant>,
  sic_text_4: Option<Variant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LimitedPartnerships {
  num_gen_partners: Option<i32>,
  num_lim_partners: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousName {
  con_date: Option<Variant>,
  company_name: Option<Variant>,
}
