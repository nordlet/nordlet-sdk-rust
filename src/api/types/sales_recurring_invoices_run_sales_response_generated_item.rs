pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecurringInvoicesRunSalesResponseGeneratedItem {
    #[serde(rename = "recurringInvoiceId")]
    #[serde(default)]
    pub recurring_invoice_id: String,
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: NaiveDate,
    #[serde(default)]
    pub issued: bool,
    #[serde(default)]
    pub sent: bool,
}

impl RecurringInvoicesRunSalesResponseGeneratedItem {
    pub fn builder() -> RecurringInvoicesRunSalesResponseGeneratedItemBuilder {
        <RecurringInvoicesRunSalesResponseGeneratedItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurringInvoicesRunSalesResponseGeneratedItemBuilder {
    recurring_invoice_id: Option<String>,
    invoice_id: Option<String>,
    issue_date: Option<NaiveDate>,
    issued: Option<bool>,
    sent: Option<bool>,
}

impl RecurringInvoicesRunSalesResponseGeneratedItemBuilder {
    pub fn recurring_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.recurring_invoice_id = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn issued(mut self, value: bool) -> Self {
        self.issued = Some(value);
        self
    }

    pub fn sent(mut self, value: bool) -> Self {
        self.sent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecurringInvoicesRunSalesResponseGeneratedItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recurring_invoice_id`](RecurringInvoicesRunSalesResponseGeneratedItemBuilder::recurring_invoice_id)
    /// - [`invoice_id`](RecurringInvoicesRunSalesResponseGeneratedItemBuilder::invoice_id)
    /// - [`issue_date`](RecurringInvoicesRunSalesResponseGeneratedItemBuilder::issue_date)
    /// - [`issued`](RecurringInvoicesRunSalesResponseGeneratedItemBuilder::issued)
    /// - [`sent`](RecurringInvoicesRunSalesResponseGeneratedItemBuilder::sent)
    pub fn build(self) -> Result<RecurringInvoicesRunSalesResponseGeneratedItem, BuildError> {
        Ok(RecurringInvoicesRunSalesResponseGeneratedItem {
            recurring_invoice_id: self
                .recurring_invoice_id
                .ok_or_else(|| BuildError::missing_field("recurring_invoice_id"))?,
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            issue_date: self
                .issue_date
                .ok_or_else(|| BuildError::missing_field("issue_date"))?,
            issued: self
                .issued
                .ok_or_else(|| BuildError::missing_field("issued"))?,
            sent: self.sent.ok_or_else(|| BuildError::missing_field("sent"))?,
        })
    }
}
