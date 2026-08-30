pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountSessionsRevokeRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1AccountSessionsRevokeRequest {
    pub fn builder() -> PostV1AccountSessionsRevokeRequestBuilder {
        <PostV1AccountSessionsRevokeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountSessionsRevokeRequestBuilder {
    id: Option<String>,
}

impl PostV1AccountSessionsRevokeRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountSessionsRevokeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountSessionsRevokeRequestBuilder::id)
    pub fn build(self) -> Result<PostV1AccountSessionsRevokeRequest, BuildError> {
        Ok(PostV1AccountSessionsRevokeRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
