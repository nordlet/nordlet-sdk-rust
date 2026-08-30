pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountSessionsRevokeOthersResponse {
    #[serde(default)]
    pub revoked: i64,
}

impl PostV1AccountSessionsRevokeOthersResponse {
    pub fn builder() -> PostV1AccountSessionsRevokeOthersResponseBuilder {
        <PostV1AccountSessionsRevokeOthersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountSessionsRevokeOthersResponseBuilder {
    revoked: Option<i64>,
}

impl PostV1AccountSessionsRevokeOthersResponseBuilder {
    pub fn revoked(mut self, value: i64) -> Self {
        self.revoked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountSessionsRevokeOthersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revoked`](PostV1AccountSessionsRevokeOthersResponseBuilder::revoked)
    pub fn build(self) -> Result<PostV1AccountSessionsRevokeOthersResponse, BuildError> {
        Ok(PostV1AccountSessionsRevokeOthersResponse {
            revoked: self
                .revoked
                .ok_or_else(|| BuildError::missing_field("revoked"))?,
        })
    }
}
