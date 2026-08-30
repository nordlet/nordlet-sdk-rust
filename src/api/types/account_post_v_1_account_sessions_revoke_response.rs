pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountSessionsRevokeResponse {
    #[serde(default)]
    pub revoked: bool,
}

impl PostV1AccountSessionsRevokeResponse {
    pub fn builder() -> PostV1AccountSessionsRevokeResponseBuilder {
        <PostV1AccountSessionsRevokeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountSessionsRevokeResponseBuilder {
    revoked: Option<bool>,
}

impl PostV1AccountSessionsRevokeResponseBuilder {
    pub fn revoked(mut self, value: bool) -> Self {
        self.revoked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountSessionsRevokeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revoked`](PostV1AccountSessionsRevokeResponseBuilder::revoked)
    pub fn build(self) -> Result<PostV1AccountSessionsRevokeResponse, BuildError> {
        Ok(PostV1AccountSessionsRevokeResponse {
            revoked: self
                .revoked
                .ok_or_else(|| BuildError::missing_field("revoked"))?,
        })
    }
}
