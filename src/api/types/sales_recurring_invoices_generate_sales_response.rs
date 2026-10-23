pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RecurringInvoicesGenerateSalesResponse {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "recurringInvoice")]
    pub recurring_invoice: RecurringInvoicesGenerateSalesResponseRecurringInvoice,
}

impl RecurringInvoicesGenerateSalesResponse {
    pub fn builder() -> RecurringInvoicesGenerateSalesResponseBuilder {
        <RecurringInvoicesGenerateSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurringInvoicesGenerateSalesResponseBuilder {
    invoice_id: Option<String>,
    issue_date: Option<NaiveDate>,
    issued: Option<bool>,
    sent: Option<bool>,
    error: Option<String>,
    recurring_invoice: Option<RecurringInvoicesGenerateSalesResponseRecurringInvoice>,
}

impl RecurringInvoicesGenerateSalesResponseBuilder {
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

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn recurring_invoice(
        mut self,
        value: RecurringInvoicesGenerateSalesResponseRecurringInvoice,
    ) -> Self {
        self.recurring_invoice = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecurringInvoicesGenerateSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](RecurringInvoicesGenerateSalesResponseBuilder::invoice_id)
    /// - [`issue_date`](RecurringInvoicesGenerateSalesResponseBuilder::issue_date)
    /// - [`issued`](RecurringInvoicesGenerateSalesResponseBuilder::issued)
    /// - [`sent`](RecurringInvoicesGenerateSalesResponseBuilder::sent)
    /// - [`recurring_invoice`](RecurringInvoicesGenerateSalesResponseBuilder::recurring_invoice)
    pub fn build(self) -> Result<RecurringInvoicesGenerateSalesResponse, BuildError> {
        Ok(RecurringInvoicesGenerateSalesResponse {
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
            error: self.error,
            recurring_invoice: self
                .recurring_invoice
                .ok_or_else(|| BuildError::missing_field("recurring_invoice"))?,
        })
    }
}
