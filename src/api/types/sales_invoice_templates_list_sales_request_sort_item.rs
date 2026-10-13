pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceTemplatesListSalesRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<InvoiceTemplatesListSalesRequestSortItemDir>,
}

impl InvoiceTemplatesListSalesRequestSortItem {
    pub fn builder() -> InvoiceTemplatesListSalesRequestSortItemBuilder {
        <InvoiceTemplatesListSalesRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceTemplatesListSalesRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<InvoiceTemplatesListSalesRequestSortItemDir>,
}

impl InvoiceTemplatesListSalesRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: InvoiceTemplatesListSalesRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoiceTemplatesListSalesRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](InvoiceTemplatesListSalesRequestSortItemBuilder::field)
    pub fn build(self) -> Result<InvoiceTemplatesListSalesRequestSortItem, BuildError> {
        Ok(InvoiceTemplatesListSalesRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
