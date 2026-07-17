pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountApiKeysListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(rename = "lastUsedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    #[serde(rename = "revokedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1AccountApiKeysListResponseRowsItem {
    pub fn builder() -> PostV1AccountApiKeysListResponseRowsItemBuilder {
        <PostV1AccountApiKeysListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountApiKeysListResponseRowsItemBuilder {
    id: Option<String>,
    name: Option<String>,
    scopes: Option<Vec<String>>,
    last_used_at: Option<String>,
    revoked_at: Option<String>,
    created_at: Option<String>,
}

impl PostV1AccountApiKeysListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn last_used_at(mut self, value: impl Into<String>) -> Self {
        self.last_used_at = Some(value.into());
        self
    }

    pub fn revoked_at(mut self, value: impl Into<String>) -> Self {
        self.revoked_at = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountApiKeysListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountApiKeysListResponseRowsItemBuilder::id)
    /// - [`name`](PostV1AccountApiKeysListResponseRowsItemBuilder::name)
    /// - [`scopes`](PostV1AccountApiKeysListResponseRowsItemBuilder::scopes)
    /// - [`created_at`](PostV1AccountApiKeysListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1AccountApiKeysListResponseRowsItem, BuildError> {
        Ok(PostV1AccountApiKeysListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
            last_used_at: self.last_used_at,
            revoked_at: self.revoked_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
