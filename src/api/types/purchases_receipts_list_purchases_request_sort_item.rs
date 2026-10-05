pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceiptsListPurchasesRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ReceiptsListPurchasesRequestSortItemDir>,
}

impl ReceiptsListPurchasesRequestSortItem {
    pub fn builder() -> ReceiptsListPurchasesRequestSortItemBuilder {
        <ReceiptsListPurchasesRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptsListPurchasesRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ReceiptsListPurchasesRequestSortItemDir>,
}

impl ReceiptsListPurchasesRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ReceiptsListPurchasesRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceiptsListPurchasesRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ReceiptsListPurchasesRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ReceiptsListPurchasesRequestSortItem, BuildError> {
        Ok(ReceiptsListPurchasesRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
