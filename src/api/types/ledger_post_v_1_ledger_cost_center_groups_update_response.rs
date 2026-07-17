pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerCostCenterGroupsUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1LedgerCostCenterGroupsUpdateResponse {
    pub fn builder() -> PostV1LedgerCostCenterGroupsUpdateResponseBuilder {
        <PostV1LedgerCostCenterGroupsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerCostCenterGroupsUpdateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    created_at: Option<String>,
}

impl PostV1LedgerCostCenterGroupsUpdateResponseBuilder {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerCostCenterGroupsUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LedgerCostCenterGroupsUpdateResponseBuilder::id)
    /// - [`code`](PostV1LedgerCostCenterGroupsUpdateResponseBuilder::code)
    /// - [`name`](PostV1LedgerCostCenterGroupsUpdateResponseBuilder::name)
    /// - [`created_at`](PostV1LedgerCostCenterGroupsUpdateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1LedgerCostCenterGroupsUpdateResponse, BuildError> {
        Ok(PostV1LedgerCostCenterGroupsUpdateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
