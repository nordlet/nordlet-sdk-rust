pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeysCreateAccountRequest {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(rename = "expiresInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_days: Option<i64>,
}

impl ApiKeysCreateAccountRequest {
    pub fn builder() -> ApiKeysCreateAccountRequestBuilder {
        <ApiKeysCreateAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeysCreateAccountRequestBuilder {
    name: Option<String>,
    scopes: Option<Vec<String>>,
    expires_in_days: Option<i64>,
}

impl ApiKeysCreateAccountRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn expires_in_days(mut self, value: i64) -> Self {
        self.expires_in_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeysCreateAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ApiKeysCreateAccountRequestBuilder::name)
    pub fn build(self) -> Result<ApiKeysCreateAccountRequest, BuildError> {
        Ok(ApiKeysCreateAccountRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            scopes: self.scopes,
            expires_in_days: self.expires_in_days,
        })
    }
}
