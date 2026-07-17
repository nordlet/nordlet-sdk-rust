pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountInvitesRevokeResponse {
    #[serde(default)]
    pub revoked: bool,
}

impl PostV1AccountInvitesRevokeResponse {
    pub fn builder() -> PostV1AccountInvitesRevokeResponseBuilder {
        <PostV1AccountInvitesRevokeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountInvitesRevokeResponseBuilder {
    revoked: Option<bool>,
}

impl PostV1AccountInvitesRevokeResponseBuilder {
    pub fn revoked(mut self, value: bool) -> Self {
        self.revoked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountInvitesRevokeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revoked`](PostV1AccountInvitesRevokeResponseBuilder::revoked)
    pub fn build(self) -> Result<PostV1AccountInvitesRevokeResponse, BuildError> {
        Ok(PostV1AccountInvitesRevokeResponse {
            revoked: self
                .revoked
                .ok_or_else(|| BuildError::missing_field("revoked"))?,
        })
    }
}
