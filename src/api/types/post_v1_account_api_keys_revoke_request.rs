pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountApiKeysRevokeRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1AccountApiKeysRevokeRequest {
    pub fn builder() -> PostV1AccountApiKeysRevokeRequestBuilder {
        <PostV1AccountApiKeysRevokeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountApiKeysRevokeRequestBuilder {
    id: Option<String>,
}

impl PostV1AccountApiKeysRevokeRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountApiKeysRevokeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountApiKeysRevokeRequestBuilder::id)
    pub fn build(self) -> Result<PostV1AccountApiKeysRevokeRequest, BuildError> {
        Ok(PostV1AccountApiKeysRevokeRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
