pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionsRevokeAccountResponse {
    #[serde(default)]
    pub revoked: bool,
}

impl SessionsRevokeAccountResponse {
    pub fn builder() -> SessionsRevokeAccountResponseBuilder {
        <SessionsRevokeAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionsRevokeAccountResponseBuilder {
    revoked: Option<bool>,
}

impl SessionsRevokeAccountResponseBuilder {
    pub fn revoked(mut self, value: bool) -> Self {
        self.revoked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionsRevokeAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revoked`](SessionsRevokeAccountResponseBuilder::revoked)
    pub fn build(self) -> Result<SessionsRevokeAccountResponse, BuildError> {
        Ok(SessionsRevokeAccountResponse {
            revoked: self
                .revoked
                .ok_or_else(|| BuildError::missing_field("revoked"))?,
        })
    }
}
