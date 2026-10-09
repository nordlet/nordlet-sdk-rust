pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeferralsListPurchasesRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<DeferralsListPurchasesRequestSortItemDir>,
}

impl DeferralsListPurchasesRequestSortItem {
    pub fn builder() -> DeferralsListPurchasesRequestSortItemBuilder {
        <DeferralsListPurchasesRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeferralsListPurchasesRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<DeferralsListPurchasesRequestSortItemDir>,
}

impl DeferralsListPurchasesRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: DeferralsListPurchasesRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeferralsListPurchasesRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](DeferralsListPurchasesRequestSortItemBuilder::field)
    pub fn build(self) -> Result<DeferralsListPurchasesRequestSortItem, BuildError> {
        Ok(DeferralsListPurchasesRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
