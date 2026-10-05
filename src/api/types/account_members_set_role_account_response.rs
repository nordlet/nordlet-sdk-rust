pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembersSetRoleAccountResponse {
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub role: String,
}

impl MembersSetRoleAccountResponse {
    pub fn builder() -> MembersSetRoleAccountResponseBuilder {
        <MembersSetRoleAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembersSetRoleAccountResponseBuilder {
    user_id: Option<String>,
    role: Option<String>,
}

impl MembersSetRoleAccountResponseBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembersSetRoleAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](MembersSetRoleAccountResponseBuilder::user_id)
    /// - [`role`](MembersSetRoleAccountResponseBuilder::role)
    pub fn build(self) -> Result<MembersSetRoleAccountResponse, BuildError> {
        Ok(MembersSetRoleAccountResponse {
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
        })
    }
}
