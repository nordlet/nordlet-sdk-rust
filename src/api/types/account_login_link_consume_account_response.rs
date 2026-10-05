pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LoginLinkConsumeAccountResponse {
    #[serde(default)]
    pub token: String,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub user: LoginLinkConsumeAccountResponseUser,
    #[serde(rename = "isNewUser")]
    #[serde(default)]
    pub is_new_user: bool,
}

impl LoginLinkConsumeAccountResponse {
    pub fn builder() -> LoginLinkConsumeAccountResponseBuilder {
        <LoginLinkConsumeAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoginLinkConsumeAccountResponseBuilder {
    token: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    user: Option<LoginLinkConsumeAccountResponseUser>,
    is_new_user: Option<bool>,
}

impl LoginLinkConsumeAccountResponseBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn user(mut self, value: LoginLinkConsumeAccountResponseUser) -> Self {
        self.user = Some(value);
        self
    }

    pub fn is_new_user(mut self, value: bool) -> Self {
        self.is_new_user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LoginLinkConsumeAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](LoginLinkConsumeAccountResponseBuilder::token)
    /// - [`expires_at`](LoginLinkConsumeAccountResponseBuilder::expires_at)
    /// - [`user`](LoginLinkConsumeAccountResponseBuilder::user)
    /// - [`is_new_user`](LoginLinkConsumeAccountResponseBuilder::is_new_user)
    pub fn build(self) -> Result<LoginLinkConsumeAccountResponse, BuildError> {
        Ok(LoginLinkConsumeAccountResponse {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
            is_new_user: self
                .is_new_user
                .ok_or_else(|| BuildError::missing_field("is_new_user"))?,
        })
    }
}
