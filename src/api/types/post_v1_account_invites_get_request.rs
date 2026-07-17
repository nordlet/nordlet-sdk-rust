pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountInvitesGetRequest {
    #[serde(default)]
    pub token: String,
}

impl PostV1AccountInvitesGetRequest {
    pub fn builder() -> PostV1AccountInvitesGetRequestBuilder {
        <PostV1AccountInvitesGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountInvitesGetRequestBuilder {
    token: Option<String>,
}

impl PostV1AccountInvitesGetRequestBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountInvitesGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](PostV1AccountInvitesGetRequestBuilder::token)
    pub fn build(self) -> Result<PostV1AccountInvitesGetRequest, BuildError> {
        Ok(PostV1AccountInvitesGetRequest {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
