pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerCostCentersUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

impl PostV1LedgerCostCentersUpdateRequest {
    pub fn builder() -> PostV1LedgerCostCentersUpdateRequestBuilder {
        <PostV1LedgerCostCentersUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerCostCentersUpdateRequestBuilder {
    id: Option<String>,
    name: Option<String>,
    is_active: Option<bool>,
    group_id: Option<String>,
}

impl PostV1LedgerCostCentersUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerCostCentersUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LedgerCostCentersUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1LedgerCostCentersUpdateRequest, BuildError> {
        Ok(PostV1LedgerCostCentersUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            is_active: self.is_active,
            group_id: self.group_id,
        })
    }
}
