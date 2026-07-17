pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerCostCentersCreateRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

impl PostV1LedgerCostCentersCreateRequest {
    pub fn builder() -> PostV1LedgerCostCentersCreateRequestBuilder {
        <PostV1LedgerCostCentersCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerCostCentersCreateRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    group_id: Option<String>,
}

impl PostV1LedgerCostCentersCreateRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerCostCentersCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1LedgerCostCentersCreateRequestBuilder::code)
    /// - [`name`](PostV1LedgerCostCentersCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerCostCentersCreateRequest, BuildError> {
        Ok(PostV1LedgerCostCentersCreateRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            group_id: self.group_id,
        })
    }
}
