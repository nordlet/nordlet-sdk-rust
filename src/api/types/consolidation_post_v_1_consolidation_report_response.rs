pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportResponse {
    #[serde(rename = "presentationCurrency")]
    #[serde(default)]
    pub presentation_currency: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    pub category: PostV1ConsolidationReportResponseCategory,
    pub statements: PostV1ConsolidationReportResponseStatements,
    #[serde(rename = "trialBalance")]
    #[serde(default)]
    pub trial_balance: Vec<PostV1ConsolidationReportResponseTrialBalanceItem>,
    #[serde(rename = "nonControllingInterest")]
    #[serde(default)]
    pub non_controlling_interest: PostV1ConsolidationReportResponseNonControllingInterest,
    #[serde(rename = "equityMethod")]
    #[serde(default)]
    pub equity_method: PostV1ConsolidationReportResponseEquityMethod,
    #[serde(default)]
    pub members: Vec<PostV1ConsolidationReportResponseMembersItem>,
    #[serde(default)]
    pub eliminations: PostV1ConsolidationReportResponseEliminations,
    #[serde(rename = "intercompanyCandidates")]
    #[serde(default)]
    pub intercompany_candidates: Vec<PostV1ConsolidationReportResponseIntercompanyCandidatesItem>,
}

impl PostV1ConsolidationReportResponse {
    pub fn builder() -> PostV1ConsolidationReportResponseBuilder {
        <PostV1ConsolidationReportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportResponseBuilder {
    presentation_currency: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
    category: Option<PostV1ConsolidationReportResponseCategory>,
    statements: Option<PostV1ConsolidationReportResponseStatements>,
    trial_balance: Option<Vec<PostV1ConsolidationReportResponseTrialBalanceItem>>,
    non_controlling_interest: Option<PostV1ConsolidationReportResponseNonControllingInterest>,
    equity_method: Option<PostV1ConsolidationReportResponseEquityMethod>,
    members: Option<Vec<PostV1ConsolidationReportResponseMembersItem>>,
    eliminations: Option<PostV1ConsolidationReportResponseEliminations>,
    intercompany_candidates:
        Option<Vec<PostV1ConsolidationReportResponseIntercompanyCandidatesItem>>,
}

impl PostV1ConsolidationReportResponseBuilder {
    pub fn presentation_currency(mut self, value: impl Into<String>) -> Self {
        self.presentation_currency = Some(value.into());
        self
    }

    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn category(mut self, value: PostV1ConsolidationReportResponseCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn statements(mut self, value: PostV1ConsolidationReportResponseStatements) -> Self {
        self.statements = Some(value);
        self
    }

    pub fn trial_balance(
        mut self,
        value: Vec<PostV1ConsolidationReportResponseTrialBalanceItem>,
    ) -> Self {
        self.trial_balance = Some(value);
        self
    }

    pub fn non_controlling_interest(
        mut self,
        value: PostV1ConsolidationReportResponseNonControllingInterest,
    ) -> Self {
        self.non_controlling_interest = Some(value);
        self
    }

    pub fn equity_method(mut self, value: PostV1ConsolidationReportResponseEquityMethod) -> Self {
        self.equity_method = Some(value);
        self
    }

    pub fn members(mut self, value: Vec<PostV1ConsolidationReportResponseMembersItem>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn eliminations(mut self, value: PostV1ConsolidationReportResponseEliminations) -> Self {
        self.eliminations = Some(value);
        self
    }

    pub fn intercompany_candidates(
        mut self,
        value: Vec<PostV1ConsolidationReportResponseIntercompanyCandidatesItem>,
    ) -> Self {
        self.intercompany_candidates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`presentation_currency`](PostV1ConsolidationReportResponseBuilder::presentation_currency)
    /// - [`from_date`](PostV1ConsolidationReportResponseBuilder::from_date)
    /// - [`to_date`](PostV1ConsolidationReportResponseBuilder::to_date)
    /// - [`category`](PostV1ConsolidationReportResponseBuilder::category)
    /// - [`statements`](PostV1ConsolidationReportResponseBuilder::statements)
    /// - [`trial_balance`](PostV1ConsolidationReportResponseBuilder::trial_balance)
    /// - [`non_controlling_interest`](PostV1ConsolidationReportResponseBuilder::non_controlling_interest)
    /// - [`equity_method`](PostV1ConsolidationReportResponseBuilder::equity_method)
    /// - [`members`](PostV1ConsolidationReportResponseBuilder::members)
    /// - [`eliminations`](PostV1ConsolidationReportResponseBuilder::eliminations)
    /// - [`intercompany_candidates`](PostV1ConsolidationReportResponseBuilder::intercompany_candidates)
    pub fn build(self) -> Result<PostV1ConsolidationReportResponse, BuildError> {
        Ok(PostV1ConsolidationReportResponse {
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
            intercompany_candidates: self
                .intercompany_candidates
                .ok_or_else(|| BuildError::missing_field("intercompany_candidates"))?,
        })
    }
}
