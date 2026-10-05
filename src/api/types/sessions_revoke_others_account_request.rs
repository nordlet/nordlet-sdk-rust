pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionsRevokeOthersAccountRequest {}

impl SessionsRevokeOthersAccountRequest {
    pub fn builder() -> SessionsRevokeOthersAccountRequestBuilder {
        <SessionsRevokeOthersAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionsRevokeOthersAccountRequestBuilder {}

impl SessionsRevokeOthersAccountRequestBuilder {
    /// Consumes the builder and constructs a [`SessionsRevokeOthersAccountRequest`].
    pub fn build(self) -> Result<SessionsRevokeOthersAccountRequest, BuildError> {
        Ok(SessionsRevokeOthersAccountRequest {})
    }
}
