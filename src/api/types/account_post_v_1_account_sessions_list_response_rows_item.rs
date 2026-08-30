pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountSessionsListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "companyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    pub expires_at: String,
    #[serde(default)]
    pub current: bool,
}

impl PostV1AccountSessionsListResponseRowsItem {
    pub fn builder() -> PostV1AccountSessionsListResponseRowsItemBuilder {
        <PostV1AccountSessionsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountSessionsListResponseRowsItemBuilder {
    id: Option<String>,
    company_id: Option<String>,
    created_at: Option<String>,
    expires_at: Option<String>,
    current: Option<bool>,
}

impl PostV1AccountSessionsListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn current(mut self, value: bool) -> Self {
        self.current = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountSessionsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountSessionsListResponseRowsItemBuilder::id)
    /// - [`created_at`](PostV1AccountSessionsListResponseRowsItemBuilder::created_at)
    /// - [`expires_at`](PostV1AccountSessionsListResponseRowsItemBuilder::expires_at)
    /// - [`current`](PostV1AccountSessionsListResponseRowsItemBuilder::current)
    pub fn build(self) -> Result<PostV1AccountSessionsListResponseRowsItem, BuildError> {
        Ok(PostV1AccountSessionsListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            company_id: self.company_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            current: self
                .current
                .ok_or_else(|| BuildError::missing_field("current"))?,
        })
    }
}
