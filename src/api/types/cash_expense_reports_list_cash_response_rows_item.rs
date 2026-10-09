pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExpenseReportsListCashResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub number: String,
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(rename = "netTotal")]
    #[serde(default)]
    pub net_total: String,
    #[serde(rename = "vatTotal")]
    #[serde(default)]
    pub vat_total: String,
    #[serde(default)]
    pub total: String,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ExpenseReportsListCashResponseRowsItem {
    pub fn builder() -> ExpenseReportsListCashResponseRowsItemBuilder {
        <ExpenseReportsListCashResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExpenseReportsListCashResponseRowsItemBuilder {
    id: Option<String>,
    number: Option<String>,
    employee_id: Option<String>,
    date: Option<NaiveDate>,
    net_total: Option<String>,
    vat_total: Option<String>,
    total: Option<String>,
    journal_transaction_id: Option<String>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ExpenseReportsListCashResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn net_total(mut self, value: impl Into<String>) -> Self {
        self.net_total = Some(value.into());
        self
    }

    pub fn vat_total(mut self, value: impl Into<String>) -> Self {
        self.vat_total = Some(value.into());
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExpenseReportsListCashResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExpenseReportsListCashResponseRowsItemBuilder::id)
    /// - [`number`](ExpenseReportsListCashResponseRowsItemBuilder::number)
    /// - [`employee_id`](ExpenseReportsListCashResponseRowsItemBuilder::employee_id)
    /// - [`date`](ExpenseReportsListCashResponseRowsItemBuilder::date)
    /// - [`net_total`](ExpenseReportsListCashResponseRowsItemBuilder::net_total)
    /// - [`vat_total`](ExpenseReportsListCashResponseRowsItemBuilder::vat_total)
    /// - [`total`](ExpenseReportsListCashResponseRowsItemBuilder::total)
    /// - [`created_at`](ExpenseReportsListCashResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<ExpenseReportsListCashResponseRowsItem, BuildError> {
        Ok(ExpenseReportsListCashResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            number: self
                .number
                .ok_or_else(|| BuildError::missing_field("number"))?,
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            net_total: self
                .net_total
                .ok_or_else(|| BuildError::missing_field("net_total"))?,
            vat_total: self
                .vat_total
                .ok_or_else(|| BuildError::missing_field("vat_total"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            journal_transaction_id: self.journal_transaction_id,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
