pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecurringInvoicesDeleteSalesResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl RecurringInvoicesDeleteSalesResponse {
    pub fn builder() -> RecurringInvoicesDeleteSalesResponseBuilder {
        <RecurringInvoicesDeleteSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurringInvoicesDeleteSalesResponseBuilder {
    deleted: Option<bool>,
}

impl RecurringInvoicesDeleteSalesResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecurringInvoicesDeleteSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](RecurringInvoicesDeleteSalesResponseBuilder::deleted)
    pub fn build(self) -> Result<RecurringInvoicesDeleteSalesResponse, BuildError> {
        Ok(RecurringInvoicesDeleteSalesResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
