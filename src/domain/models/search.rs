
// TODO Implement
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchCaseParams {
    pub _filed_date_to: Option<String>,
    pub _cause_of_action: Option<Vec<String>>,
    pub _nature_suit: Option<Vec<String>>,
    pub _last_business_name: Option<String>,
    pub _first_name: Option<String>,
    pub _middle_name: Option<String>,
    pub _type_field: Option<String>,
    pub _exact_matches_only: Option<bool>,
}

impl SearchCaseParams {
    pub fn build_query(&self) -> (String, Vec<ParameterValue>) {
        let mut conditions = Vec::new();
        let mut params = Vec::new();
        let mut param_count = 1;

        if let Some(date_to) = &self.filed_date_to {
            conditions.push(format!("filed_date <= ${}", param_count));
            params.push(ParameterValue::Str(date_to.clone()));
            param_count += 1;
        }

        // Add conditions for other fields similarly
        if let Some(name) = &self.last_business_name {
            conditions.push(format!("last_business_name ILIKE ${}", param_count));
            params.push(ParameterValue::Str(format!("%{}%", name)));
            param_count += 1;
        }

        // Continue with other fields...

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        (where_clause, params)
    }
}
