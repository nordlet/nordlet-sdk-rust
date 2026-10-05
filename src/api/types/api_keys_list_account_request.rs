pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeysListAccountRequest {}

impl ApiKeysListAccountRequest {
    pub fn builder() -> ApiKeysListAccountRequestBuilder {
        <ApiKeysListAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeysListAccountRequestBuilder {}

impl ApiKeysListAccountRequestBuilder {
    /// Consumes the builder and constructs a [`ApiKeysListAccountRequest`].
    pub fn build(self) -> Result<ApiKeysListAccountRequest, BuildError> {
        Ok(ApiKeysListAccountRequest {})
    }
}
