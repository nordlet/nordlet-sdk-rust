pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeysCreateAccountResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default)]
    pub key: String,
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expires_at: Option<DateTime<FixedOffset>>,
}

impl ApiKeysCreateAccountResponse {
    pub fn builder() -> ApiKeysCreateAccountResponseBuilder {
        <ApiKeysCreateAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeysCreateAccountResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    scopes: Option<Vec<String>>,
    key: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl ApiKeysCreateAccountResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeysCreateAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ApiKeysCreateAccountResponseBuilder::id)
    /// - [`name`](ApiKeysCreateAccountResponseBuilder::name)
    /// - [`scopes`](ApiKeysCreateAccountResponseBuilder::scopes)
    /// - [`key`](ApiKeysCreateAccountResponseBuilder::key)
    pub fn build(self) -> Result<ApiKeysCreateAccountResponse, BuildError> {
        Ok(ApiKeysCreateAccountResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            expires_at: self.expires_at,
        })
    }
}
