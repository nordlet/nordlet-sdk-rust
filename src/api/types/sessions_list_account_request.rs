pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionsListAccountRequest {}

impl SessionsListAccountRequest {
    pub fn builder() -> SessionsListAccountRequestBuilder {
        <SessionsListAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionsListAccountRequestBuilder {}

impl SessionsListAccountRequestBuilder {
    /// Consumes the builder and constructs a [`SessionsListAccountRequest`].
    pub fn build(self) -> Result<SessionsListAccountRequest, BuildError> {
        Ok(SessionsListAccountRequest {})
    }
}
