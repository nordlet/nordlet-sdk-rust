pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecurringInvoicesDeleteSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl RecurringInvoicesDeleteSalesRequest {
    pub fn builder() -> RecurringInvoicesDeleteSalesRequestBuilder {
        <RecurringInvoicesDeleteSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurringInvoicesDeleteSalesRequestBuilder {
    id: Option<String>,
}

impl RecurringInvoicesDeleteSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecurringInvoicesDeleteSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RecurringInvoicesDeleteSalesRequestBuilder::id)
    pub fn build(self) -> Result<RecurringInvoicesDeleteSalesRequest, BuildError> {
        Ok(RecurringInvoicesDeleteSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
