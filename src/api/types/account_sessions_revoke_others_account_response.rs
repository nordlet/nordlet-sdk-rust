pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionsRevokeOthersAccountResponse {
    #[serde(default)]
    pub revoked: i64,
}

impl SessionsRevokeOthersAccountResponse {
    pub fn builder() -> SessionsRevokeOthersAccountResponseBuilder {
        <SessionsRevokeOthersAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionsRevokeOthersAccountResponseBuilder {
    revoked: Option<i64>,
}

impl SessionsRevokeOthersAccountResponseBuilder {
    pub fn revoked(mut self, value: i64) -> Self {
        self.revoked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionsRevokeOthersAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revoked`](SessionsRevokeOthersAccountResponseBuilder::revoked)
    pub fn build(self) -> Result<SessionsRevokeOthersAccountResponse, BuildError> {
        Ok(SessionsRevokeOthersAccountResponse {
            revoked: self
                .revoked
                .ok_or_else(|| BuildError::missing_field("revoked"))?,
        })
    }
}
