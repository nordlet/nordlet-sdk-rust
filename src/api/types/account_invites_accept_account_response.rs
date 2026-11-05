pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitesAcceptAccountResponse {
    #[serde(default)]
    pub token: String,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    #[serde(rename = "twoFactorRequired")]
    #[serde(default)]
    pub two_factor_required: bool,
    #[serde(default)]
    pub user: InvitesAcceptAccountResponseUser,
}

impl InvitesAcceptAccountResponse {
    pub fn builder() -> InvitesAcceptAccountResponseBuilder {
        <InvitesAcceptAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitesAcceptAccountResponseBuilder {
    token: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    two_factor_required: Option<bool>,
    user: Option<InvitesAcceptAccountResponseUser>,
}

impl InvitesAcceptAccountResponseBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn two_factor_required(mut self, value: bool) -> Self {
        self.two_factor_required = Some(value);
        self
    }

    pub fn user(mut self, value: InvitesAcceptAccountResponseUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvitesAcceptAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](InvitesAcceptAccountResponseBuilder::token)
    /// - [`expires_at`](InvitesAcceptAccountResponseBuilder::expires_at)
    /// - [`two_factor_required`](InvitesAcceptAccountResponseBuilder::two_factor_required)
    /// - [`user`](InvitesAcceptAccountResponseBuilder::user)
    pub fn build(self) -> Result<InvitesAcceptAccountResponse, BuildError> {
        Ok(InvitesAcceptAccountResponse {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            two_factor_required: self
                .two_factor_required
                .ok_or_else(|| BuildError::missing_field("two_factor_required"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
