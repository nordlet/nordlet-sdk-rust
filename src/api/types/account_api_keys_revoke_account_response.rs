pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeysRevokeAccountResponse {
    #[serde(default)]
    pub revoked: bool,
}

impl ApiKeysRevokeAccountResponse {
    pub fn builder() -> ApiKeysRevokeAccountResponseBuilder {
        <ApiKeysRevokeAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeysRevokeAccountResponseBuilder {
    revoked: Option<bool>,
}

impl ApiKeysRevokeAccountResponseBuilder {
    pub fn revoked(mut self, value: bool) -> Self {
        self.revoked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeysRevokeAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revoked`](ApiKeysRevokeAccountResponseBuilder::revoked)
    pub fn build(self) -> Result<ApiKeysRevokeAccountResponse, BuildError> {
        Ok(ApiKeysRevokeAccountResponse {
            revoked: self
                .revoked
                .ok_or_else(|| BuildError::missing_field("revoked"))?,
        })
    }
}
