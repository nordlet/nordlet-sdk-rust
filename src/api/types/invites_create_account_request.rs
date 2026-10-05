pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvitesCreateAccountRequest {
    #[serde(default)]
    pub email: String,
    pub role: InvitesCreateAccountRequestRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<InvitesCreateAccountRequestLocale>,
}

impl InvitesCreateAccountRequest {
    pub fn builder() -> InvitesCreateAccountRequestBuilder {
        <InvitesCreateAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitesCreateAccountRequestBuilder {
    email: Option<String>,
    role: Option<InvitesCreateAccountRequestRole>,
    locale: Option<InvitesCreateAccountRequestLocale>,
}

impl InvitesCreateAccountRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn role(mut self, value: InvitesCreateAccountRequestRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn locale(mut self, value: InvitesCreateAccountRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvitesCreateAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](InvitesCreateAccountRequestBuilder::email)
    /// - [`role`](InvitesCreateAccountRequestBuilder::role)
    pub fn build(self) -> Result<InvitesCreateAccountRequest, BuildError> {
        Ok(InvitesCreateAccountRequest {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            locale: self.locale,
        })
    }
}
