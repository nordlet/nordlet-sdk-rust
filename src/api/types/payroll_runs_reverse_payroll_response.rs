pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RunsReversePayrollResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "payDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_date: Option<NaiveDate>,
    pub status: RunsReversePayrollResponseStatus,
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
    pub component_totals: Vec<RunsReversePayrollResponseComponentTotalsItem>,
    #[serde(rename = "netTotal")]
    #[serde(default)]
    pub net_total: String,
    #[serde(rename = "paidAmount")]
    #[serde(default)]
    pub paid_amount: String,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "approvedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub approved_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "reversedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub reversed_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "reversalJournalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal_journal_transaction_id: Option<String>,
    #[serde(rename = "reversalReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal_reason: Option<String>,
}

impl RunsReversePayrollResponse {
    pub fn builder() -> RunsReversePayrollResponseBuilder {
        <RunsReversePayrollResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsReversePayrollResponseBuilder {
    id: Option<String>,
    year: Option<i64>,
    month: Option<i64>,
    country_code: Option<String>,
    pay_date: Option<NaiveDate>,
    status: Option<RunsReversePayrollResponseStatus>,
    gross_total: Option<String>,
    tax_allowance_total: Option<String>,
    income_tax_total: Option<String>,
    employee_contributions_total: Option<String>,
    employer_contributions_total: Option<String>,
    component_totals: Option<Vec<RunsReversePayrollResponseComponentTotalsItem>>,
    net_total: Option<String>,
    paid_amount: Option<String>,
    journal_transaction_id: Option<String>,
    notes: Option<String>,
    warnings: Option<Vec<String>>,
    created_at: Option<DateTime<FixedOffset>>,
    approved_at: Option<DateTime<FixedOffset>>,
    reversed_at: Option<DateTime<FixedOffset>>,
    reversal_journal_transaction_id: Option<String>,
    reversal_reason: Option<String>,
}

impl RunsReversePayrollResponseBuilder {
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

    pub fn pay_date(mut self, value: NaiveDate) -> Self {
        self.pay_date = Some(value);
        self
    }

    pub fn status(mut self, value: RunsReversePayrollResponseStatus) -> Self {
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
        value: Vec<RunsReversePayrollResponseComponentTotalsItem>,
    ) -> Self {
        self.component_totals = Some(value);
        self
    }

    pub fn net_total(mut self, value: impl Into<String>) -> Self {
        self.net_total = Some(value.into());
        self
    }

    pub fn paid_amount(mut self, value: impl Into<String>) -> Self {
        self.paid_amount = Some(value.into());
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

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn approved_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.approved_at = Some(value);
        self
    }

    pub fn reversed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.reversed_at = Some(value);
        self
    }

    pub fn reversal_journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.reversal_journal_transaction_id = Some(value.into());
        self
    }

    pub fn reversal_reason(mut self, value: impl Into<String>) -> Self {
        self.reversal_reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunsReversePayrollResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RunsReversePayrollResponseBuilder::id)
    /// - [`year`](RunsReversePayrollResponseBuilder::year)
    /// - [`month`](RunsReversePayrollResponseBuilder::month)
    /// - [`country_code`](RunsReversePayrollResponseBuilder::country_code)
    /// - [`status`](RunsReversePayrollResponseBuilder::status)
    /// - [`gross_total`](RunsReversePayrollResponseBuilder::gross_total)
    /// - [`tax_allowance_total`](RunsReversePayrollResponseBuilder::tax_allowance_total)
    /// - [`income_tax_total`](RunsReversePayrollResponseBuilder::income_tax_total)
    /// - [`employee_contributions_total`](RunsReversePayrollResponseBuilder::employee_contributions_total)
    /// - [`employer_contributions_total`](RunsReversePayrollResponseBuilder::employer_contributions_total)
    /// - [`component_totals`](RunsReversePayrollResponseBuilder::component_totals)
    /// - [`net_total`](RunsReversePayrollResponseBuilder::net_total)
    /// - [`paid_amount`](RunsReversePayrollResponseBuilder::paid_amount)
    /// - [`warnings`](RunsReversePayrollResponseBuilder::warnings)
    /// - [`created_at`](RunsReversePayrollResponseBuilder::created_at)
    pub fn build(self) -> Result<RunsReversePayrollResponse, BuildError> {
        Ok(RunsReversePayrollResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            pay_date: self.pay_date,
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
            paid_amount: self
                .paid_amount
                .ok_or_else(|| BuildError::missing_field("paid_amount"))?,
            journal_transaction_id: self.journal_transaction_id,
            notes: self.notes,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            approved_at: self.approved_at,
            reversed_at: self.reversed_at,
            reversal_journal_transaction_id: self.reversal_journal_transaction_id,
            reversal_reason: self.reversal_reason,
        })
    }
}
