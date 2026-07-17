pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountInvitesListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub role: String,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    pub expires_at: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub expired: bool,
}

impl PostV1AccountInvitesListResponseRowsItem {
    pub fn builder() -> PostV1AccountInvitesListResponseRowsItemBuilder {
        <PostV1AccountInvitesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountInvitesListResponseRowsItemBuilder {
    id: Option<String>,
    email: Option<String>,
    role: Option<String>,
    expires_at: Option<String>,
    created_at: Option<String>,
    expired: Option<bool>,
}

impl PostV1AccountInvitesListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn expired(mut self, value: bool) -> Self {
        self.expired = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountInvitesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountInvitesListResponseRowsItemBuilder::id)
    /// - [`email`](PostV1AccountInvitesListResponseRowsItemBuilder::email)
    /// - [`role`](PostV1AccountInvitesListResponseRowsItemBuilder::role)
    /// - [`expires_at`](PostV1AccountInvitesListResponseRowsItemBuilder::expires_at)
    /// - [`created_at`](PostV1AccountInvitesListResponseRowsItemBuilder::created_at)
    /// - [`expired`](PostV1AccountInvitesListResponseRowsItemBuilder::expired)
    pub fn build(self) -> Result<PostV1AccountInvitesListResponseRowsItem, BuildError> {
        Ok(PostV1AccountInvitesListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            expired: self
                .expired
                .ok_or_else(|| BuildError::missing_field("expired"))?,
        })
    }
}
