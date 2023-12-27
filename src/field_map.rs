use std::collections::HashMap;

pub struct FieldMap {
  map: HashMap<String, String>,
}

impl FieldMap {
  pub fn new() -> Self {
    let mut field_map = FieldMap {
      map: HashMap::new(),
    };
    field_map.add_fields();
    field_map
  }

  pub fn get(&self, field: &str) -> String {
    match self.map.get(field) {
      Some(mapped_field) => mapped_field.clone(),
      None => field.to_string(),
    }
  }

  fn add(&mut self, k: &str, v: &str) {
    self.map.insert(k.to_string(), v.to_string());
  }

  fn add_fields(&mut self) {
    self.add("CompanyName", "company_name");
    self.add("CompanyNumber", "company_number");
    self.add("RegAddress.CareOf", "reg_address.care_of");
    self.add("RegAddress.POBox", "reg_address.po_box");
    self.add("RegAddress.AddressLine1", "reg_address.address_line_1");
    self.add("RegAddress.AddressLine2", "reg_address.address_line_2");
    self.add("RegAddress.PostTown", "reg_address.post_town");
    self.add("RegAddress.County", "reg_address.county");
    self.add("RegAddress.Country", "reg_address.country");
    self.add("RegAddress.PostCode", "reg_address.post_code");
    self.add("CompanyCategory", "company_category");
    self.add("CompanyStatus", "company_status");
    self.add("CountryOfOrigin", "country_of_origin");
    self.add("DissolutionDate", "dissolution_date");
    self.add("IncorporationDate", "incorporation_date");
    self.add("Accounts.AccountRefDay", "accounts.account_ref_day");
    self.add("Accounts.AccountRefMonth", "accounts.account_ref_month");
    self.add("Accounts.NextDueDate", "accounts.next_due_date");
    self.add("Accounts.LastMadeUpDate", "accounts.last_made_up_date");
    self.add("Accounts.AccountCategory", "accounts.account_category");
    self.add("Returns.NextDueDate", "returns.next_due_date");
    self.add("Returns.LastMadeUpDate", "returns.last_made_up_date");
    self.add("Mortgages.NumMortCharges", "mortgages.num_mort_charges");
    self.add("Mortgages.NumMortOutstanding", "mortgages.num_mort_outstanding");
    self.add("Mortgages.NumMortPartSatisfied", "mortgages.num_mort_part_satisfied");
    self.add("Mortgages.NumMortSatisfied", "mortgages.num_mort_satisfied");
    self.add("SICCode.SicText_1", "sic_code.sic_text_1");
    self.add("SICCode.SicText_2", "sic_code.sic_text_2");
    self.add("SICCode.SicText_3", "sic_code.sic_text_3");
    self.add("SICCode.SicText_4", "sic_code.sic_text_4");
    self.add("LimitedPartnerships.NumGenPartners", "limited_partnerships.num_gen_partners");
    self.add("LimitedPartnerships.NumLimPartners", "limited_partnerships.num_lim_partners");
    self.add("URI", "uri");
    self.add("PreviousName_1.CONDATE", "previous_name_1.con_date");
    self.add("PreviousName_1.CompanyName", "previous_name_1.company_name");
    self.add("PreviousName_2.CONDATE", "previous_name_2.con_date");
    self.add("PreviousName_2.CompanyName", "previous_name_2.company_name");
    self.add("PreviousName_3.CONDATE", "previous_name_3.con_date");
    self.add("PreviousName_3.CompanyName", "previous_name_3.company_name");
    self.add("PreviousName_4.CONDATE", "previous_name_4.con_date");
    self.add("PreviousName_4.CompanyName", "previous_name_4.company_name");
    self.add("PreviousName_5.CONDATE", "previous_name_5.con_date");
    self.add("PreviousName_5.CompanyName", "previous_name_5.company_name");
    self.add("PreviousName_6.CONDATE", "previous_name_6.con_date");
    self.add("PreviousName_6.CompanyName", "previous_name_6.company_name");
    self.add("PreviousName_7.CONDATE", "previous_name_7.con_date");
    self.add("PreviousName_7.CompanyName", "previous_name_7.company_name");
    self.add("PreviousName_8.CONDATE", "previous_name_8.con_date");
    self.add("PreviousName_8.CompanyName", "previous_name_8.company_name");
    self.add("PreviousName_9.CONDATE", "previous_name_9.con_date");
    self.add("PreviousName_9.CompanyName", "previous_name_9.company_name");
    self.add("PreviousName_10.CONDATE", "previous_name_10.con_date");
    self.add("PreviousName_10.CompanyName", "previous_name_10.company_name");
    self.add("ConfStmtNextDueDate", "conf_stmt_next_due_date");
    self.add("ConfStmtLastMadeUpDate", "conf_stmt_last_made_up_date");
  }
}