pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountInvitesAcceptRequest {
    #[serde(default)]
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<PostV1AccountInvitesAcceptRequestLocale>,
}

impl PostV1AccountInvitesAcceptRequest {
    pub fn builder() -> PostV1AccountInvitesAcceptRequestBuilder {
        <PostV1AccountInvitesAcceptRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountInvitesAcceptRequestBuilder {
    token: Option<String>,
    name: Option<String>,
    locale: Option<PostV1AccountInvitesAcceptRequestLocale>,
}

impl PostV1AccountInvitesAcceptRequestBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn locale(mut self, value: PostV1AccountInvitesAcceptRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountInvitesAcceptRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](PostV1AccountInvitesAcceptRequestBuilder::token)
    pub fn build(self) -> Result<PostV1AccountInvitesAcceptRequest, BuildError> {
        Ok(PostV1AccountInvitesAcceptRequest {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
            name: self.name,
            locale: self.locale,
        })
    }
}
