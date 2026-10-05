pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundLiabilityListSalesRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<RefundLiabilityListSalesRequestSortItemDir>,
}

impl RefundLiabilityListSalesRequestSortItem {
    pub fn builder() -> RefundLiabilityListSalesRequestSortItemBuilder {
        <RefundLiabilityListSalesRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundLiabilityListSalesRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<RefundLiabilityListSalesRequestSortItemDir>,
}

impl RefundLiabilityListSalesRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: RefundLiabilityListSalesRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundLiabilityListSalesRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](RefundLiabilityListSalesRequestSortItemBuilder::field)
    pub fn build(self) -> Result<RefundLiabilityListSalesRequestSortItem, BuildError> {
        Ok(RefundLiabilityListSalesRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
