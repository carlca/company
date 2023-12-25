use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Company {
  #[serde(rename = "CompanyName")]
  company_name: String,
  #[serde(rename = "CompanyNumber")]
  company_number: u32,
  #[serde(rename = "RegAddress")]
  reg_address: RegAddress,
  #[serde(rename = "CompanyCategory")]
  company_category: String,
  #[serde(rename = "CompanyStatus")]
  company_status: String,
  #[serde(rename = "CountryOfOrigin")]
  country_of_origin: String,
  #[serde(rename = "DissolutionDate")]
  dissolution_date: String,
  #[serde(rename = "IncorporationDate")]
  incorporation_date: String,
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
  uri: String,
  #[serde(rename = "PreviousName_1")]
  previous_name_1: PreviousName,
  #[serde(rename = "PreviousName_2")]
  previous_name_2: PreviousName,
  #[serde(rename = "PreviousName_3")]
  previous_name_3: PreviousName,
  #[serde(rename = "PreviousName_4")]
  previous_name_4: PreviousName,
  #[serde(rename = "PreviousName_5")]
  previous_name_5: PreviousName,
  #[serde(rename = "PreviousName_6")]
  previous_name_6: PreviousName,
  #[serde(rename = "PreviousName_7")]
  previous_name_7: PreviousName,
  #[serde(rename = "PreviousName_8")]
  previous_name_8: PreviousName,
  #[serde(rename = "PreviousName_9")]
  previous_name_9: PreviousName,
  #[serde(rename = "PreviousName_10")]
  previous_name_10: PreviousName,
  #[serde(rename = "ConfStmtNextDueDate")]
  conf_stmt_next_due_date: String,
  #[serde(rename = "confStmtLastMadeUpDate")]
  conf_stmt_last_made_up_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RegAddress {
  #[serde(rename = "CareOf")]
  care_of: String,
  #[serde(rename = "POBox")]
  po_box: String,
  #[serde(rename = "AddressLine1")]
  address_line_1: String,
  #[serde(rename = "AddressLine2")]
  address_line_2: String,
  #[serde(rename = "PostTown")]
  post_town: String,
  #[serde(rename = "County")]
  county: String,
  #[serde(rename = "Country")]
  country: String,
  #[serde(rename = "PostCode")]
  post_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Accounts {
  #[serde(rename = "AccountRefDay")]
  account_ref_day: u8,
  #[serde(rename = "AccountRefMonth")]
  account_ref_month: u8,
  #[serde(rename = "NextDueDate")]
  next_due_date: String,
  #[serde(rename = "LastMadeUpDate")]
  last_made_up_date: String,
  #[serde(rename = "AccountCategory")]
  account_category: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Returns {
  #[serde(rename = "NextDueDate")]
  next_due_date: String,
  #[serde(rename = "LastMadeUpDate")]
  last_made_up_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Mortgages {
  #[serde(rename = "NumMortCharges")]
  num_mort_charges: u16,
  #[serde(rename = "NumMortOutstanding")]
  num_mort_outstanding: u16,
  #[serde(rename = "NumMortPartSatisfied")]
  num_mort_part_satisfied: u16,
  #[serde(rename = "NumMortSatisfied")]
  num_mort_satisfied: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct SICCode {
  #[serde(rename = "SicText_1")]
  sic_text_1: String,
  #[serde(rename = "SicText_2")]
  sic_text_2: String,
  #[serde(rename = "SicText_3")]
  sic_text_3: String,
  #[serde(rename = "SicText_4")]
  sic_text_4: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LimitedPartnerships {
  #[serde(rename = "NumGenPartners")]
  num_gen_partners: u32,
  #[serde(rename = "NumLimPartners")]
  num_lim_partners: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct PreviousName {
  #[serde(rename = "CONDATE")]
  con_date: String,
  #[serde(rename = "CompanyName")]
  company_name: String,
}