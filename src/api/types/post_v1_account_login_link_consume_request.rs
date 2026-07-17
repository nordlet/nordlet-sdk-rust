pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountLoginLinkConsumeRequest {
    #[serde(default)]
    pub token: String,
}

impl PostV1AccountLoginLinkConsumeRequest {
    pub fn builder() -> PostV1AccountLoginLinkConsumeRequestBuilder {
        <PostV1AccountLoginLinkConsumeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountLoginLinkConsumeRequestBuilder {
    token: Option<String>,
}

impl PostV1AccountLoginLinkConsumeRequestBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountLoginLinkConsumeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](PostV1AccountLoginLinkConsumeRequestBuilder::token)
    pub fn build(self) -> Result<PostV1AccountLoginLinkConsumeRequest, BuildError> {
        Ok(PostV1AccountLoginLinkConsumeRequest {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
