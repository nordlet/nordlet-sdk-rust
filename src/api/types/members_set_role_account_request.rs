pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MembersSetRoleAccountRequest {
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
    pub role: MembersSetRoleAccountRequestRole,
}

impl MembersSetRoleAccountRequest {
    pub fn builder() -> MembersSetRoleAccountRequestBuilder {
        <MembersSetRoleAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembersSetRoleAccountRequestBuilder {
    user_id: Option<String>,
    role: Option<MembersSetRoleAccountRequestRole>,
}

impl MembersSetRoleAccountRequestBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn role(mut self, value: MembersSetRoleAccountRequestRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MembersSetRoleAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](MembersSetRoleAccountRequestBuilder::user_id)
    /// - [`role`](MembersSetRoleAccountRequestBuilder::role)
    pub fn build(self) -> Result<MembersSetRoleAccountRequest, BuildError> {
        Ok(MembersSetRoleAccountRequest {
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
        })
    }
}
