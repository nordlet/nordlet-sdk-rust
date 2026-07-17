pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountInvitesRevokeRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1AccountInvitesRevokeRequest {
    pub fn builder() -> PostV1AccountInvitesRevokeRequestBuilder {
        <PostV1AccountInvitesRevokeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountInvitesRevokeRequestBuilder {
    id: Option<String>,
}

impl PostV1AccountInvitesRevokeRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountInvitesRevokeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountInvitesRevokeRequestBuilder::id)
    pub fn build(self) -> Result<PostV1AccountInvitesRevokeRequest, BuildError> {
        Ok(PostV1AccountInvitesRevokeRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
