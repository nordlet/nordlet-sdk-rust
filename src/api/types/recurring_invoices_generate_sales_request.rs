pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecurringInvoicesGenerateSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl RecurringInvoicesGenerateSalesRequest {
    pub fn builder() -> RecurringInvoicesGenerateSalesRequestBuilder {
        <RecurringInvoicesGenerateSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurringInvoicesGenerateSalesRequestBuilder {
    id: Option<String>,
}

impl RecurringInvoicesGenerateSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecurringInvoicesGenerateSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RecurringInvoicesGenerateSalesRequestBuilder::id)
    pub fn build(self) -> Result<RecurringInvoicesGenerateSalesRequest, BuildError> {
        Ok(RecurringInvoicesGenerateSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
