pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountApiKeysRevokeResponse {
    #[serde(default)]
    pub revoked: bool,
}

impl PostV1AccountApiKeysRevokeResponse {
    pub fn builder() -> PostV1AccountApiKeysRevokeResponseBuilder {
        <PostV1AccountApiKeysRevokeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountApiKeysRevokeResponseBuilder {
    revoked: Option<bool>,
}

impl PostV1AccountApiKeysRevokeResponseBuilder {
    pub fn revoked(mut self, value: bool) -> Self {
        self.revoked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountApiKeysRevokeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revoked`](PostV1AccountApiKeysRevokeResponseBuilder::revoked)
    pub fn build(self) -> Result<PostV1AccountApiKeysRevokeResponse, BuildError> {
        Ok(PostV1AccountApiKeysRevokeResponse {
            revoked: self
                .revoked
                .ok_or_else(|| BuildError::missing_field("revoked"))?,
        })
    }
}
