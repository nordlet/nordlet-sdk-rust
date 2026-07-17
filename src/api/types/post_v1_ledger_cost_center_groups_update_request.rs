pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerCostCenterGroupsUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PostV1LedgerCostCenterGroupsUpdateRequest {
    pub fn builder() -> PostV1LedgerCostCenterGroupsUpdateRequestBuilder {
        <PostV1LedgerCostCenterGroupsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerCostCenterGroupsUpdateRequestBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
}

impl PostV1LedgerCostCenterGroupsUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerCostCenterGroupsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LedgerCostCenterGroupsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1LedgerCostCenterGroupsUpdateRequest, BuildError> {
        Ok(PostV1LedgerCostCenterGroupsUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code,
            name: self.name,
        })
    }
}
