pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecurringInvoicesListSalesRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<RecurringInvoicesListSalesRequestSortItemDir>,
}

impl RecurringInvoicesListSalesRequestSortItem {
    pub fn builder() -> RecurringInvoicesListSalesRequestSortItemBuilder {
        <RecurringInvoicesListSalesRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurringInvoicesListSalesRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<RecurringInvoicesListSalesRequestSortItemDir>,
}

impl RecurringInvoicesListSalesRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: RecurringInvoicesListSalesRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecurringInvoicesListSalesRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](RecurringInvoicesListSalesRequestSortItemBuilder::field)
    pub fn build(self) -> Result<RecurringInvoicesListSalesRequestSortItem, BuildError> {
        Ok(RecurringInvoicesListSalesRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
