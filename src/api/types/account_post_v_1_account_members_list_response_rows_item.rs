pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountMembersListResponseRowsItem {
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub role: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1AccountMembersListResponseRowsItem {
    pub fn builder() -> PostV1AccountMembersListResponseRowsItemBuilder {
        <PostV1AccountMembersListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountMembersListResponseRowsItemBuilder {
    user_id: Option<String>,
    email: Option<String>,
    name: Option<String>,
    role: Option<String>,
    created_at: Option<String>,
}

impl PostV1AccountMembersListResponseRowsItemBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountMembersListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](PostV1AccountMembersListResponseRowsItemBuilder::user_id)
    /// - [`email`](PostV1AccountMembersListResponseRowsItemBuilder::email)
    /// - [`role`](PostV1AccountMembersListResponseRowsItemBuilder::role)
    /// - [`created_at`](PostV1AccountMembersListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1AccountMembersListResponseRowsItem, BuildError> {
        Ok(PostV1AccountMembersListResponseRowsItem {
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
