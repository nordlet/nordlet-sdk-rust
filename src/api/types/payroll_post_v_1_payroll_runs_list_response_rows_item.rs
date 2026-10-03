pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    pub status: PostV1PayrollRunsListResponseRowsItemStatus,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "taxAllowanceTotal")]
    #[serde(default)]
    pub tax_allowance_total: String,
    #[serde(rename = "incomeTaxTotal")]
    #[serde(default)]
    pub income_tax_total: String,
    #[serde(rename = "employeeContributionsTotal")]
    #[serde(default)]
    pub employee_contributions_total: String,
    #[serde(rename = "employerContributionsTotal")]
    #[serde(default)]
    pub employer_contributions_total: String,
    #[serde(rename = "componentTotals")]
    #[serde(default)]
    pub component_totals: Vec<PostV1PayrollRunsListResponseRowsItemComponentTotalsItem>,
    #[serde(rename = "netTotal")]
    #[serde(default)]
    pub net_total: String,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "approvedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<String>,
}

impl PostV1PayrollRunsListResponseRowsItem {
    pub fn builder() -> PostV1PayrollRunsListResponseRowsItemBuilder {
        <PostV1PayrollRunsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsListResponseRowsItemBuilder {
    id: Option<String>,
    year: Option<i64>,
    month: Option<i64>,
    country_code: Option<String>,
    status: Option<PostV1PayrollRunsListResponseRowsItemStatus>,
    gross_total: Option<String>,
    tax_allowance_total: Option<String>,
    income_tax_total: Option<String>,
    employee_contributions_total: Option<String>,
    employer_contributions_total: Option<String>,
    component_totals: Option<Vec<PostV1PayrollRunsListResponseRowsItemComponentTotalsItem>>,
    net_total: Option<String>,
    journal_transaction_id: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
    approved_at: Option<String>,
}

impl PostV1PayrollRunsListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1PayrollRunsListResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    pub fn tax_allowance_total(mut self, value: impl Into<String>) -> Self {
        self.tax_allowance_total = Some(value.into());
        self
    }

    pub fn income_tax_total(mut self, value: impl Into<String>) -> Self {
        self.income_tax_total = Some(value.into());
        self
    }

    pub fn employee_contributions_total(mut self, value: impl Into<String>) -> Self {
        self.employee_contributions_total = Some(value.into());
        self
    }

    pub fn employer_contributions_total(mut self, value: impl Into<String>) -> Self {
        self.employer_contributions_total = Some(value.into());
        self
    }

    pub fn component_totals(
        mut self,
        value: Vec<PostV1PayrollRunsListResponseRowsItemComponentTotalsItem>,
    ) -> Self {
        self.component_totals = Some(value);
        self
    }

    pub fn net_total(mut self, value: impl Into<String>) -> Self {
        self.net_total = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn approved_at(mut self, value: impl Into<String>) -> Self {
        self.approved_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PayrollRunsListResponseRowsItemBuilder::id)
    /// - [`year`](PostV1PayrollRunsListResponseRowsItemBuilder::year)
    /// - [`month`](PostV1PayrollRunsListResponseRowsItemBuilder::month)
    /// - [`country_code`](PostV1PayrollRunsListResponseRowsItemBuilder::country_code)
    /// - [`status`](PostV1PayrollRunsListResponseRowsItemBuilder::status)
    /// - [`gross_total`](PostV1PayrollRunsListResponseRowsItemBuilder::gross_total)
    /// - [`tax_allowance_total`](PostV1PayrollRunsListResponseRowsItemBuilder::tax_allowance_total)
    /// - [`income_tax_total`](PostV1PayrollRunsListResponseRowsItemBuilder::income_tax_total)
    /// - [`employee_contributions_total`](PostV1PayrollRunsListResponseRowsItemBuilder::employee_contributions_total)
    /// - [`employer_contributions_total`](PostV1PayrollRunsListResponseRowsItemBuilder::employer_contributions_total)
    /// - [`component_totals`](PostV1PayrollRunsListResponseRowsItemBuilder::component_totals)
    /// - [`net_total`](PostV1PayrollRunsListResponseRowsItemBuilder::net_total)
    /// - [`created_at`](PostV1PayrollRunsListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1PayrollRunsListResponseRowsItem, BuildError> {
        Ok(PostV1PayrollRunsListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            gross_total: self
                .gross_total
                .ok_or_else(|| BuildError::missing_field("gross_total"))?,
            tax_allowance_total: self
                .tax_allowance_total
                .ok_or_else(|| BuildError::missing_field("tax_allowance_total"))?,
            income_tax_total: self
                .income_tax_total
                .ok_or_else(|| BuildError::missing_field("income_tax_total"))?,
            employee_contributions_total: self
                .employee_contributions_total
                .ok_or_else(|| BuildError::missing_field("employee_contributions_total"))?,
            employer_contributions_total: self
                .employer_contributions_total
                .ok_or_else(|| BuildError::missing_field("employer_contributions_total"))?,
            component_totals: self
                .component_totals
                .ok_or_else(|| BuildError::missing_field("component_totals"))?,
            net_total: self
                .net_total
                .ok_or_else(|| BuildError::missing_field("net_total"))?,
            journal_transaction_id: self.journal_transaction_id,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            approved_at: self.approved_at,
        })
    }
}
