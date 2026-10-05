pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitesRevokeAccountResponse {
    #[serde(default)]
    pub revoked: bool,
}

impl InvitesRevokeAccountResponse {
    pub fn builder() -> InvitesRevokeAccountResponseBuilder {
        <InvitesRevokeAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitesRevokeAccountResponseBuilder {
    revoked: Option<bool>,
}

impl InvitesRevokeAccountResponseBuilder {
    pub fn revoked(mut self, value: bool) -> Self {
        self.revoked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvitesRevokeAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revoked`](InvitesRevokeAccountResponseBuilder::revoked)
    pub fn build(self) -> Result<InvitesRevokeAccountResponse, BuildError> {
        Ok(InvitesRevokeAccountResponse {
            revoked: self
                .revoked
                .ok_or_else(|| BuildError::missing_field("revoked"))?,
        })
    }
}
