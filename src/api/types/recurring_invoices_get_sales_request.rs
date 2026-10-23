pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecurringInvoicesGetSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl RecurringInvoicesGetSalesRequest {
    pub fn builder() -> RecurringInvoicesGetSalesRequestBuilder {
        <RecurringInvoicesGetSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurringInvoicesGetSalesRequestBuilder {
    id: Option<String>,
}

impl RecurringInvoicesGetSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecurringInvoicesGetSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RecurringInvoicesGetSalesRequestBuilder::id)
    pub fn build(self) -> Result<RecurringInvoicesGetSalesRequest, BuildError> {
        Ok(RecurringInvoicesGetSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
