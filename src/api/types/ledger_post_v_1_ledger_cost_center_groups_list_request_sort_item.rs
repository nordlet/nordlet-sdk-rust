pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerCostCenterGroupsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1LedgerCostCenterGroupsListRequestSortItemDir>,
}

impl PostV1LedgerCostCenterGroupsListRequestSortItem {
    pub fn builder() -> PostV1LedgerCostCenterGroupsListRequestSortItemBuilder {
        <PostV1LedgerCostCenterGroupsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerCostCenterGroupsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1LedgerCostCenterGroupsListRequestSortItemDir>,
}

impl PostV1LedgerCostCenterGroupsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1LedgerCostCenterGroupsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerCostCenterGroupsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1LedgerCostCenterGroupsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1LedgerCostCenterGroupsListRequestSortItem, BuildError> {
        Ok(PostV1LedgerCostCenterGroupsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
