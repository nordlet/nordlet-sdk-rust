pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeferralsListPurchasesResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "invoiceLineId")]
    #[serde(default)]
    pub invoice_line_id: String,
    #[serde(rename = "scheduleDate")]
    #[serde(default)]
    pub schedule_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub amount: String,
    #[serde(rename = "expenseAccountCode")]
    #[serde(default)]
    pub expense_account_code: String,
    #[serde(rename = "prepaidAccountCode")]
    #[serde(default)]
    pub prepaid_account_code: String,
    pub status: DeferralsListPurchasesResponseRowsItemStatus,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
}

impl DeferralsListPurchasesResponseRowsItem {
    pub fn builder() -> DeferralsListPurchasesResponseRowsItemBuilder {
        <DeferralsListPurchasesResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeferralsListPurchasesResponseRowsItemBuilder {
    id: Option<String>,
    invoice_id: Option<String>,
    invoice_line_id: Option<String>,
    schedule_date: Option<NaiveDate>,
    description: Option<String>,
    amount: Option<String>,
    expense_account_code: Option<String>,
    prepaid_account_code: Option<String>,
    status: Option<DeferralsListPurchasesResponseRowsItemStatus>,
    journal_transaction_id: Option<String>,
}

impl DeferralsListPurchasesResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn invoice_line_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_line_id = Some(value.into());
        self
    }

    pub fn schedule_date(mut self, value: NaiveDate) -> Self {
        self.schedule_date = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn expense_account_code(mut self, value: impl Into<String>) -> Self {
        self.expense_account_code = Some(value.into());
        self
    }

    pub fn prepaid_account_code(mut self, value: impl Into<String>) -> Self {
        self.prepaid_account_code = Some(value.into());
        self
    }

    pub fn status(mut self, value: DeferralsListPurchasesResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeferralsListPurchasesResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeferralsListPurchasesResponseRowsItemBuilder::id)
    /// - [`invoice_id`](DeferralsListPurchasesResponseRowsItemBuilder::invoice_id)
    /// - [`invoice_line_id`](DeferralsListPurchasesResponseRowsItemBuilder::invoice_line_id)
    /// - [`schedule_date`](DeferralsListPurchasesResponseRowsItemBuilder::schedule_date)
    /// - [`amount`](DeferralsListPurchasesResponseRowsItemBuilder::amount)
    /// - [`expense_account_code`](DeferralsListPurchasesResponseRowsItemBuilder::expense_account_code)
    /// - [`prepaid_account_code`](DeferralsListPurchasesResponseRowsItemBuilder::prepaid_account_code)
    /// - [`status`](DeferralsListPurchasesResponseRowsItemBuilder::status)
    pub fn build(self) -> Result<DeferralsListPurchasesResponseRowsItem, BuildError> {
        Ok(DeferralsListPurchasesResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            invoice_line_id: self
                .invoice_line_id
                .ok_or_else(|| BuildError::missing_field("invoice_line_id"))?,
            schedule_date: self
                .schedule_date
                .ok_or_else(|| BuildError::missing_field("schedule_date"))?,
            description: self.description,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            expense_account_code: self
                .expense_account_code
                .ok_or_else(|| BuildError::missing_field("expense_account_code"))?,
            prepaid_account_code: self
                .prepaid_account_code
                .ok_or_else(|| BuildError::missing_field("prepaid_account_code"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            journal_transaction_id: self.journal_transaction_id,
        })
    }
}
