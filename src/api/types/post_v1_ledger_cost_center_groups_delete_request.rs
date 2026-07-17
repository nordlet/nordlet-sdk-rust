pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerCostCenterGroupsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1LedgerCostCenterGroupsDeleteRequest {
    pub fn builder() -> PostV1LedgerCostCenterGroupsDeleteRequestBuilder {
        <PostV1LedgerCostCenterGroupsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerCostCenterGroupsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1LedgerCostCenterGroupsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerCostCenterGroupsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LedgerCostCenterGroupsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1LedgerCostCenterGroupsDeleteRequest, BuildError> {
        Ok(PostV1LedgerCostCenterGroupsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
