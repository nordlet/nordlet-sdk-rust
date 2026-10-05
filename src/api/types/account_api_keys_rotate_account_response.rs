pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeysRotateAccountResponse {
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
    #[serde(rename = "replacedKeyId")]
    #[serde(default)]
    pub replaced_key_id: String,
    #[serde(rename = "replacedKeyExpiresAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub replaced_key_expires_at: DateTime<FixedOffset>,
}

impl ApiKeysRotateAccountResponse {
    pub fn builder() -> ApiKeysRotateAccountResponseBuilder {
        <ApiKeysRotateAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeysRotateAccountResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    scopes: Option<Vec<String>>,
    key: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    replaced_key_id: Option<String>,
    replaced_key_expires_at: Option<DateTime<FixedOffset>>,
}

impl ApiKeysRotateAccountResponseBuilder {
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

    pub fn replaced_key_id(mut self, value: impl Into<String>) -> Self {
        self.replaced_key_id = Some(value.into());
        self
    }

    pub fn replaced_key_expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.replaced_key_expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeysRotateAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ApiKeysRotateAccountResponseBuilder::id)
    /// - [`name`](ApiKeysRotateAccountResponseBuilder::name)
    /// - [`scopes`](ApiKeysRotateAccountResponseBuilder::scopes)
    /// - [`key`](ApiKeysRotateAccountResponseBuilder::key)
    /// - [`replaced_key_id`](ApiKeysRotateAccountResponseBuilder::replaced_key_id)
    /// - [`replaced_key_expires_at`](ApiKeysRotateAccountResponseBuilder::replaced_key_expires_at)
    pub fn build(self) -> Result<ApiKeysRotateAccountResponse, BuildError> {
        Ok(ApiKeysRotateAccountResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            expires_at: self.expires_at,
            replaced_key_id: self
                .replaced_key_id
                .ok_or_else(|| BuildError::missing_field("replaced_key_id"))?,
            replaced_key_expires_at: self
                .replaced_key_expires_at
                .ok_or_else(|| BuildError::missing_field("replaced_key_expires_at"))?,
        })
    }
}
