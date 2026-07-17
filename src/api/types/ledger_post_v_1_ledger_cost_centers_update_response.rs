pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerCostCentersUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1LedgerCostCentersUpdateResponse {
    pub fn builder() -> PostV1LedgerCostCentersUpdateResponseBuilder {
        <PostV1LedgerCostCentersUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerCostCentersUpdateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    group_id: Option<String>,
    group_name: Option<String>,
    is_active: Option<bool>,
    created_at: Option<String>,
}

impl PostV1LedgerCostCentersUpdateResponseBuilder {
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

    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn group_name(mut self, value: impl Into<String>) -> Self {
        self.group_name = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerCostCentersUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LedgerCostCentersUpdateResponseBuilder::id)
    /// - [`code`](PostV1LedgerCostCentersUpdateResponseBuilder::code)
    /// - [`name`](PostV1LedgerCostCentersUpdateResponseBuilder::name)
    /// - [`is_active`](PostV1LedgerCostCentersUpdateResponseBuilder::is_active)
    /// - [`created_at`](PostV1LedgerCostCentersUpdateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1LedgerCostCentersUpdateResponse, BuildError> {
        Ok(PostV1LedgerCostCentersUpdateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            group_id: self.group_id,
            group_name: self.group_name,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
