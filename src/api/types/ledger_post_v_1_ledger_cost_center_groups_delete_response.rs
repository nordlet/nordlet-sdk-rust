pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerCostCenterGroupsDeleteResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl PostV1LedgerCostCenterGroupsDeleteResponse {
    pub fn builder() -> PostV1LedgerCostCenterGroupsDeleteResponseBuilder {
        <PostV1LedgerCostCenterGroupsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerCostCenterGroupsDeleteResponseBuilder {
    deleted: Option<bool>,
}

impl PostV1LedgerCostCenterGroupsDeleteResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerCostCenterGroupsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](PostV1LedgerCostCenterGroupsDeleteResponseBuilder::deleted)
    pub fn build(self) -> Result<PostV1LedgerCostCenterGroupsDeleteResponse, BuildError> {
        Ok(PostV1LedgerCostCenterGroupsDeleteResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
