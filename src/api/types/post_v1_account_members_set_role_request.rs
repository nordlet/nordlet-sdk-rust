pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AccountMembersSetRoleRequest {
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
    pub role: PostV1AccountMembersSetRoleRequestRole,
}

impl PostV1AccountMembersSetRoleRequest {
    pub fn builder() -> PostV1AccountMembersSetRoleRequestBuilder {
        <PostV1AccountMembersSetRoleRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountMembersSetRoleRequestBuilder {
    user_id: Option<String>,
    role: Option<PostV1AccountMembersSetRoleRequestRole>,
}

impl PostV1AccountMembersSetRoleRequestBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn role(mut self, value: PostV1AccountMembersSetRoleRequestRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountMembersSetRoleRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](PostV1AccountMembersSetRoleRequestBuilder::user_id)
    /// - [`role`](PostV1AccountMembersSetRoleRequestBuilder::role)
    pub fn build(self) -> Result<PostV1AccountMembersSetRoleRequest, BuildError> {
        Ok(PostV1AccountMembersSetRoleRequest {
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
        })
    }
}
