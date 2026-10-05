pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitesCreateAccountResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub role: String,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    #[serde(rename = "emailSent")]
    #[serde(default)]
    pub email_sent: bool,
}

impl InvitesCreateAccountResponse {
    pub fn builder() -> InvitesCreateAccountResponseBuilder {
        <InvitesCreateAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitesCreateAccountResponseBuilder {
    id: Option<String>,
    email: Option<String>,
    role: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    email_sent: Option<bool>,
}

impl InvitesCreateAccountResponseBuilder {
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

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn email_sent(mut self, value: bool) -> Self {
        self.email_sent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvitesCreateAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvitesCreateAccountResponseBuilder::id)
    /// - [`email`](InvitesCreateAccountResponseBuilder::email)
    /// - [`role`](InvitesCreateAccountResponseBuilder::role)
    /// - [`expires_at`](InvitesCreateAccountResponseBuilder::expires_at)
    /// - [`email_sent`](InvitesCreateAccountResponseBuilder::email_sent)
    pub fn build(self) -> Result<InvitesCreateAccountResponse, BuildError> {
        Ok(InvitesCreateAccountResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            email_sent: self
                .email_sent
                .ok_or_else(|| BuildError::missing_field("email_sent"))?,
        })
    }
}
