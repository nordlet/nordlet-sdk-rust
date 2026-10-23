pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecurringInvoicesRunSalesResponse {
    #[serde(default)]
    pub generated: Vec<RecurringInvoicesRunSalesResponseGeneratedItem>,
    #[serde(default)]
    pub errors: Vec<RecurringInvoicesRunSalesResponseErrorsItem>,
}

impl RecurringInvoicesRunSalesResponse {
    pub fn builder() -> RecurringInvoicesRunSalesResponseBuilder {
        <RecurringInvoicesRunSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurringInvoicesRunSalesResponseBuilder {
    generated: Option<Vec<RecurringInvoicesRunSalesResponseGeneratedItem>>,
    errors: Option<Vec<RecurringInvoicesRunSalesResponseErrorsItem>>,
}

impl RecurringInvoicesRunSalesResponseBuilder {
    pub fn generated(mut self, value: Vec<RecurringInvoicesRunSalesResponseGeneratedItem>) -> Self {
        self.generated = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<RecurringInvoicesRunSalesResponseErrorsItem>) -> Self {
        self.errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecurringInvoicesRunSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`generated`](RecurringInvoicesRunSalesResponseBuilder::generated)
    /// - [`errors`](RecurringInvoicesRunSalesResponseBuilder::errors)
    pub fn build(self) -> Result<RecurringInvoicesRunSalesResponse, BuildError> {
        Ok(RecurringInvoicesRunSalesResponse {
            generated: self
                .generated
                .ok_or_else(|| BuildError::missing_field("generated"))?,
            errors: self
                .errors
                .ok_or_else(|| BuildError::missing_field("errors"))?,
        })
    }
}
