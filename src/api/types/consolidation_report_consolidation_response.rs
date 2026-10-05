pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponse {
    #[serde(rename = "presentationCurrency")]
    #[serde(default)]
    pub presentation_currency: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    pub category: ReportConsolidationResponseCategory,
    pub statements: ReportConsolidationResponseStatements,
    #[serde(rename = "trialBalance")]
    #[serde(default)]
    pub trial_balance: Vec<ReportConsolidationResponseTrialBalanceItem>,
    #[serde(rename = "nonControllingInterest")]
    #[serde(default)]
    pub non_controlling_interest: ReportConsolidationResponseNonControllingInterest,
    #[serde(rename = "equityMethod")]
    #[serde(default)]
    pub equity_method: ReportConsolidationResponseEquityMethod,
    #[serde(default)]
    pub members: Vec<ReportConsolidationResponseMembersItem>,
    #[serde(default)]
    pub eliminations: ReportConsolidationResponseEliminations,
    #[serde(rename = "cashFlow")]
    #[serde(default)]
    pub cash_flow: ReportConsolidationResponseCashFlow,
    #[serde(rename = "intercompanyCandidates")]
    #[serde(default)]
    pub intercompany_candidates: Vec<ReportConsolidationResponseIntercompanyCandidatesItem>,
}

impl ReportConsolidationResponse {
    pub fn builder() -> ReportConsolidationResponseBuilder {
        <ReportConsolidationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseBuilder {
    presentation_currency: Option<String>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    category: Option<ReportConsolidationResponseCategory>,
    statements: Option<ReportConsolidationResponseStatements>,
    trial_balance: Option<Vec<ReportConsolidationResponseTrialBalanceItem>>,
    non_controlling_interest: Option<ReportConsolidationResponseNonControllingInterest>,
    equity_method: Option<ReportConsolidationResponseEquityMethod>,
    members: Option<Vec<ReportConsolidationResponseMembersItem>>,
    eliminations: Option<ReportConsolidationResponseEliminations>,
    cash_flow: Option<ReportConsolidationResponseCashFlow>,
    intercompany_candidates: Option<Vec<ReportConsolidationResponseIntercompanyCandidatesItem>>,
}

impl ReportConsolidationResponseBuilder {
    pub fn presentation_currency(mut self, value: impl Into<String>) -> Self {
        self.presentation_currency = Some(value.into());
        self
    }

    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn category(mut self, value: ReportConsolidationResponseCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn statements(mut self, value: ReportConsolidationResponseStatements) -> Self {
        self.statements = Some(value);
        self
    }

    pub fn trial_balance(
        mut self,
        value: Vec<ReportConsolidationResponseTrialBalanceItem>,
    ) -> Self {
        self.trial_balance = Some(value);
        self
    }

    pub fn non_controlling_interest(
        mut self,
        value: ReportConsolidationResponseNonControllingInterest,
    ) -> Self {
        self.non_controlling_interest = Some(value);
        self
    }

    pub fn equity_method(mut self, value: ReportConsolidationResponseEquityMethod) -> Self {
        self.equity_method = Some(value);
        self
    }

    pub fn members(mut self, value: Vec<ReportConsolidationResponseMembersItem>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn eliminations(mut self, value: ReportConsolidationResponseEliminations) -> Self {
        self.eliminations = Some(value);
        self
    }

    pub fn cash_flow(mut self, value: ReportConsolidationResponseCashFlow) -> Self {
        self.cash_flow = Some(value);
        self
    }

    pub fn intercompany_candidates(
        mut self,
        value: Vec<ReportConsolidationResponseIntercompanyCandidatesItem>,
    ) -> Self {
        self.intercompany_candidates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportConsolidationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`presentation_currency`](ReportConsolidationResponseBuilder::presentation_currency)
    /// - [`from_date`](ReportConsolidationResponseBuilder::from_date)
    /// - [`to_date`](ReportConsolidationResponseBuilder::to_date)
    /// - [`category`](ReportConsolidationResponseBuilder::category)
    /// - [`statements`](ReportConsolidationResponseBuilder::statements)
    /// - [`trial_balance`](ReportConsolidationResponseBuilder::trial_balance)
    /// - [`non_controlling_interest`](ReportConsolidationResponseBuilder::non_controlling_interest)
    /// - [`equity_method`](ReportConsolidationResponseBuilder::equity_method)
    /// - [`members`](ReportConsolidationResponseBuilder::members)
    /// - [`eliminations`](ReportConsolidationResponseBuilder::eliminations)
    /// - [`cash_flow`](ReportConsolidationResponseBuilder::cash_flow)
    /// - [`intercompany_candidates`](ReportConsolidationResponseBuilder::intercompany_candidates)
    pub fn build(self) -> Result<ReportConsolidationResponse, BuildError> {
        Ok(ReportConsolidationResponse {
            presentation_currency: self
                .presentation_currency
                .ok_or_else(|| BuildError::missing_field("presentation_currency"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            statements: self
                .statements
                .ok_or_else(|| BuildError::missing_field("statements"))?,
            trial_balance: self
                .trial_balance
                .ok_or_else(|| BuildError::missing_field("trial_balance"))?,
            non_controlling_interest: self
                .non_controlling_interest
                .ok_or_else(|| BuildError::missing_field("non_controlling_interest"))?,
            equity_method: self
                .equity_method
                .ok_or_else(|| BuildError::missing_field("equity_method"))?,
            members: self
                .members
                .ok_or_else(|| BuildError::missing_field("members"))?,
            eliminations: self
                .eliminations
                .ok_or_else(|| BuildError::missing_field("eliminations"))?,
            cash_flow: self
                .cash_flow
                .ok_or_else(|| BuildError::missing_field("cash_flow"))?,
            intercompany_candidates: self
                .intercompany_candidates
                .ok_or_else(|| BuildError::missing_field("intercompany_candidates"))?,
        })
    }
}
