pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitesGetAccountResponse {
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub role: String,
    #[serde(rename = "companyName")]
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub expired: bool,
    #[serde(rename = "userExists")]
    #[serde(default)]
    pub user_exists: bool,
}

impl InvitesGetAccountResponse {
    pub fn builder() -> InvitesGetAccountResponseBuilder {
        <InvitesGetAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitesGetAccountResponseBuilder {
    email: Option<String>,
    role: Option<String>,
    company_name: Option<String>,
    expired: Option<bool>,
    user_exists: Option<bool>,
}

impl InvitesGetAccountResponseBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn expired(mut self, value: bool) -> Self {
        self.expired = Some(value);
        self
    }

    pub fn user_exists(mut self, value: bool) -> Self {
        self.user_exists = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvitesGetAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](InvitesGetAccountResponseBuilder::email)
    /// - [`role`](InvitesGetAccountResponseBuilder::role)
    /// - [`company_name`](InvitesGetAccountResponseBuilder::company_name)
    /// - [`expired`](InvitesGetAccountResponseBuilder::expired)
    /// - [`user_exists`](InvitesGetAccountResponseBuilder::user_exists)
    pub fn build(self) -> Result<InvitesGetAccountResponse, BuildError> {
        Ok(InvitesGetAccountResponse {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            company_name: self
                .company_name
                .ok_or_else(|| BuildError::missing_field("company_name"))?,
            expired: self
                .expired
                .ok_or_else(|| BuildError::missing_field("expired"))?,
            user_exists: self
                .user_exists
                .ok_or_else(|| BuildError::missing_field("user_exists"))?,
        })
    }
}
