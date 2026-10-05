pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProfileUpdateAccountResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ProfileUpdateAccountResponse {
    pub fn builder() -> ProfileUpdateAccountResponseBuilder {
        <ProfileUpdateAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProfileUpdateAccountResponseBuilder {
    id: Option<String>,
    email: Option<String>,
    name: Option<String>,
}

impl ProfileUpdateAccountResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    /// Consumes the builder and constructs a [`ProfileUpdateAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProfileUpdateAccountResponseBuilder::id)
    /// - [`email`](ProfileUpdateAccountResponseBuilder::email)
    pub fn build(self) -> Result<ProfileUpdateAccountResponse, BuildError> {
        Ok(ProfileUpdateAccountResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
        })
    }
}
