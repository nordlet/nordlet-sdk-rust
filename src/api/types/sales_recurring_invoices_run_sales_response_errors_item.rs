pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecurringInvoicesRunSalesResponseErrorsItem {
    #[serde(rename = "recurringInvoiceId")]
    #[serde(default)]
    pub recurring_invoice_id: String,
    #[serde(default)]
    pub message: String,
}

impl RecurringInvoicesRunSalesResponseErrorsItem {
    pub fn builder() -> RecurringInvoicesRunSalesResponseErrorsItemBuilder {
        <RecurringInvoicesRunSalesResponseErrorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurringInvoicesRunSalesResponseErrorsItemBuilder {
    recurring_invoice_id: Option<String>,
    message: Option<String>,
}

impl RecurringInvoicesRunSalesResponseErrorsItemBuilder {
    pub fn recurring_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.recurring_invoice_id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecurringInvoicesRunSalesResponseErrorsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recurring_invoice_id`](RecurringInvoicesRunSalesResponseErrorsItemBuilder::recurring_invoice_id)
    /// - [`message`](RecurringInvoicesRunSalesResponseErrorsItemBuilder::message)
    pub fn build(self) -> Result<RecurringInvoicesRunSalesResponseErrorsItem, BuildError> {
        Ok(RecurringInvoicesRunSalesResponseErrorsItem {
            recurring_invoice_id: self
                .recurring_invoice_id
                .ok_or_else(|| BuildError::missing_field("recurring_invoice_id"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
