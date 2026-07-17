pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountInvitesAcceptResponse {
    #[serde(default)]
    pub token: String,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    pub expires_at: String,
    #[serde(default)]
    pub user: PostV1AccountInvitesAcceptResponseUser,
}

impl PostV1AccountInvitesAcceptResponse {
    pub fn builder() -> PostV1AccountInvitesAcceptResponseBuilder {
        <PostV1AccountInvitesAcceptResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountInvitesAcceptResponseBuilder {
    token: Option<String>,
    expires_at: Option<String>,
    user: Option<PostV1AccountInvitesAcceptResponseUser>,
}

impl PostV1AccountInvitesAcceptResponseBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn user(mut self, value: PostV1AccountInvitesAcceptResponseUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountInvitesAcceptResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](PostV1AccountInvitesAcceptResponseBuilder::token)
    /// - [`expires_at`](PostV1AccountInvitesAcceptResponseBuilder::expires_at)
    /// - [`user`](PostV1AccountInvitesAcceptResponseBuilder::user)
    pub fn build(self) -> Result<PostV1AccountInvitesAcceptResponse, BuildError> {
        Ok(PostV1AccountInvitesAcceptResponse {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
