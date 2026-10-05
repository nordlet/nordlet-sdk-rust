pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesListSalesRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<InvoicesListSalesRequestSortItemDir>,
}

impl InvoicesListSalesRequestSortItem {
    pub fn builder() -> InvoicesListSalesRequestSortItemBuilder {
        <InvoicesListSalesRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesListSalesRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<InvoicesListSalesRequestSortItemDir>,
}

impl InvoicesListSalesRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: InvoicesListSalesRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesListSalesRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](InvoicesListSalesRequestSortItemBuilder::field)
    pub fn build(self) -> Result<InvoicesListSalesRequestSortItem, BuildError> {
        Ok(InvoicesListSalesRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
