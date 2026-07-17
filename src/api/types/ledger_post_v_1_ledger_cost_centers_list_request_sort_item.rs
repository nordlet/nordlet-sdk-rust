pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerCostCentersListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1LedgerCostCentersListRequestSortItemDir>,
}

impl PostV1LedgerCostCentersListRequestSortItem {
    pub fn builder() -> PostV1LedgerCostCentersListRequestSortItemBuilder {
        <PostV1LedgerCostCentersListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerCostCentersListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1LedgerCostCentersListRequestSortItemDir>,
}

impl PostV1LedgerCostCentersListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1LedgerCostCentersListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerCostCentersListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1LedgerCostCentersListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1LedgerCostCentersListRequestSortItem, BuildError> {
        Ok(PostV1LedgerCostCentersListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
