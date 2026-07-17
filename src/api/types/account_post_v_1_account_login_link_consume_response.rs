pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountLoginLinkConsumeResponse {
    #[serde(default)]
    pub token: String,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    pub expires_at: String,
    #[serde(default)]
    pub user: PostV1AccountLoginLinkConsumeResponseUser,
    #[serde(rename = "isNewUser")]
    #[serde(default)]
    pub is_new_user: bool,
}

impl PostV1AccountLoginLinkConsumeResponse {
    pub fn builder() -> PostV1AccountLoginLinkConsumeResponseBuilder {
        <PostV1AccountLoginLinkConsumeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountLoginLinkConsumeResponseBuilder {
    token: Option<String>,
    expires_at: Option<String>,
    user: Option<PostV1AccountLoginLinkConsumeResponseUser>,
    is_new_user: Option<bool>,
}

impl PostV1AccountLoginLinkConsumeResponseBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn user(mut self, value: PostV1AccountLoginLinkConsumeResponseUser) -> Self {
        self.user = Some(value);
        self
    }

    pub fn is_new_user(mut self, value: bool) -> Self {
        self.is_new_user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountLoginLinkConsumeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](PostV1AccountLoginLinkConsumeResponseBuilder::token)
    /// - [`expires_at`](PostV1AccountLoginLinkConsumeResponseBuilder::expires_at)
    /// - [`user`](PostV1AccountLoginLinkConsumeResponseBuilder::user)
    /// - [`is_new_user`](PostV1AccountLoginLinkConsumeResponseBuilder::is_new_user)
    pub fn build(self) -> Result<PostV1AccountLoginLinkConsumeResponse, BuildError> {
        Ok(PostV1AccountLoginLinkConsumeResponse {
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
