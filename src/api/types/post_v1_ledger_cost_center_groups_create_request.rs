pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerCostCenterGroupsCreateRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerCostCenterGroupsCreateRequest {
    pub fn builder() -> PostV1LedgerCostCenterGroupsCreateRequestBuilder {
        <PostV1LedgerCostCenterGroupsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerCostCenterGroupsCreateRequestBuilder {
    code: Option<String>,
    name: Option<String>,
}

impl PostV1LedgerCostCenterGroupsCreateRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerCostCenterGroupsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1LedgerCostCenterGroupsCreateRequestBuilder::code)
    /// - [`name`](PostV1LedgerCostCenterGroupsCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerCostCenterGroupsCreateRequest, BuildError> {
        Ok(PostV1LedgerCostCenterGroupsCreateRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
