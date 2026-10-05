pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeysRevokeAccountRequest {
    #[serde(default)]
    pub id: String,
}

impl ApiKeysRevokeAccountRequest {
    pub fn builder() -> ApiKeysRevokeAccountRequestBuilder {
        <ApiKeysRevokeAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeysRevokeAccountRequestBuilder {
    id: Option<String>,
}

impl ApiKeysRevokeAccountRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApiKeysRevokeAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ApiKeysRevokeAccountRequestBuilder::id)
    pub fn build(self) -> Result<ApiKeysRevokeAccountRequest, BuildError> {
        Ok(ApiKeysRevokeAccountRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
