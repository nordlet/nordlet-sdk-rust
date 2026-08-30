pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountExportResponseSessionsItem {
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

impl PostV1AccountExportResponseSessionsItem {
    pub fn builder() -> PostV1AccountExportResponseSessionsItemBuilder {
        <PostV1AccountExportResponseSessionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountExportResponseSessionsItemBuilder {
    id: Option<String>,
    company_id: Option<String>,
    created_at: Option<String>,
    expires_at: Option<String>,
    current: Option<bool>,
}

impl PostV1AccountExportResponseSessionsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1AccountExportResponseSessionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountExportResponseSessionsItemBuilder::id)
    /// - [`created_at`](PostV1AccountExportResponseSessionsItemBuilder::created_at)
    /// - [`expires_at`](PostV1AccountExportResponseSessionsItemBuilder::expires_at)
    /// - [`current`](PostV1AccountExportResponseSessionsItemBuilder::current)
    pub fn build(self) -> Result<PostV1AccountExportResponseSessionsItem, BuildError> {
        Ok(PostV1AccountExportResponseSessionsItem {
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
