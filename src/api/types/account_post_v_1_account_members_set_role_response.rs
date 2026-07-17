pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountMembersSetRoleResponse {
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub role: String,
}

impl PostV1AccountMembersSetRoleResponse {
    pub fn builder() -> PostV1AccountMembersSetRoleResponseBuilder {
        <PostV1AccountMembersSetRoleResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountMembersSetRoleResponseBuilder {
    user_id: Option<String>,
    role: Option<String>,
}

impl PostV1AccountMembersSetRoleResponseBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountMembersSetRoleResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](PostV1AccountMembersSetRoleResponseBuilder::user_id)
    /// - [`role`](PostV1AccountMembersSetRoleResponseBuilder::role)
    pub fn build(self) -> Result<PostV1AccountMembersSetRoleResponse, BuildError> {
        Ok(PostV1AccountMembersSetRoleResponse {
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
        })
    }
}
