pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LoginLinkConsumeAccountRequest {
    #[serde(default)]
    pub token: String,
}

impl LoginLinkConsumeAccountRequest {
    pub fn builder() -> LoginLinkConsumeAccountRequestBuilder {
        <LoginLinkConsumeAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoginLinkConsumeAccountRequestBuilder {
    token: Option<String>,
}

impl LoginLinkConsumeAccountRequestBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LoginLinkConsumeAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](LoginLinkConsumeAccountRequestBuilder::token)
    pub fn build(self) -> Result<LoginLinkConsumeAccountRequest, BuildError> {
        Ok(LoginLinkConsumeAccountRequest {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
