use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Case {
    pub id: i64,
    pub case_number: String,
    pub title: String,
    pub case_type: String,
    pub nature_of_suit: String,
    pub filing_type: String,
    pub status: String,
    pub filed_date: String,
    pub closed_date: Option<String>,
    pub reopened_date: Option<String>,
    pub court_id: i64,
    pub division_id: Option<i64>,
    pub assigned_judge_id: Option<i64>,
    pub security_level: String,
    pub created_by: i64,
    pub updated_by: Option<i64>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseCreate {
    pub title: String,
    pub case_type: String,
    pub nature_of_suit: String,
    pub filing_type: String,
    pub court_id: i64,
    pub division_id: Option<i64>,
    pub assigned_judge_id: Option<i64>,
    pub security_level: String,
    pub created_by: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CaseStatus {
    PENDING,
    ACTIVE,
    CLOSED,
}


#[derive(Debug)]
pub struct CreateCaseParams {
    pub title: String,
    pub status: String,
    pub filed_date: String,
    pub court_id: i64,
    pub case_type: String,
    pub filing_type: String,
    pub security_level: String,
    pub user_id: i64,
    pub nature_of_suit: Option<String>,
    pub assigned_judge_id: Option<i64>,
    pub jury_demand: Option<String>,
    pub jurisdictional_basis: Option<String>,
    pub demand_amount: Option<f64>,
}

impl CreateCaseParams {
    pub fn new(
        title: String,
        status: String,
        filed_date: String,
        court_id: i64,
        case_type: String,
        filing_type: String,
        security_level: String,
        user_id: i64,
    ) -> Self {
        Self {
            title,
            status,
            filed_date,
            court_id,
            case_type,
            filing_type,
            security_level,
            user_id,
            nature_of_suit: None,
            assigned_judge_id: None,
            jury_demand: None,
            jurisdictional_basis: None,
            demand_amount: None,
        }
    }

    pub fn with_nature_of_suit(mut self, nature_of_suit: String) -> Self {
        self.nature_of_suit = Some(nature_of_suit);
        self
    }

    pub fn with_assigned_judge(mut self, judge_id: i64) -> Self {
        self.assigned_judge_id = Some(judge_id);
        self
    }

    pub fn with_demand_amount(mut self, amount: f64) -> Self {
        self.demand_amount = Some(amount);
        self
    }

    pub fn with_jury_demand(mut self, demand: String) -> Self {
        self.jury_demand = Some(demand);
        self
    }

    pub fn with_jurisdictional_basis(mut self, basis: String) -> Self {
        self.jurisdictional_basis = Some(basis);
        self
    }
}
