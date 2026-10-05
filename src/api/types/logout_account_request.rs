pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LogoutAccountRequest {}

impl LogoutAccountRequest {
    pub fn builder() -> LogoutAccountRequestBuilder {
        <LogoutAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LogoutAccountRequestBuilder {}

impl LogoutAccountRequestBuilder {
    /// Consumes the builder and constructs a [`LogoutAccountRequest`].
    pub fn build(self) -> Result<LogoutAccountRequest, BuildError> {
        Ok(LogoutAccountRequest {})
    }
}
