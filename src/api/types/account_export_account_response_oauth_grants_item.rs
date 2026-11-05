pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportAccountResponseOauthGrantsItem {
    #[serde(rename = "apiKeyId")]
    #[serde(default)]
    pub api_key_id: String,
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "clientName")]
    #[serde(default)]
    pub client_name: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ExportAccountResponseOauthGrantsItem {
    pub fn builder() -> ExportAccountResponseOauthGrantsItemBuilder {
        <ExportAccountResponseOauthGrantsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportAccountResponseOauthGrantsItemBuilder {
    api_key_id: Option<String>,
    client_id: Option<String>,
    client_name: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ExportAccountResponseOauthGrantsItemBuilder {
    pub fn api_key_id(mut self, value: impl Into<String>) -> Self {
        self.api_key_id = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn client_name(mut self, value: impl Into<String>) -> Self {
        self.client_name = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportAccountResponseOauthGrantsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key_id`](ExportAccountResponseOauthGrantsItemBuilder::api_key_id)
    /// - [`client_id`](ExportAccountResponseOauthGrantsItemBuilder::client_id)
    /// - [`client_name`](ExportAccountResponseOauthGrantsItemBuilder::client_name)
    /// - [`created_at`](ExportAccountResponseOauthGrantsItemBuilder::created_at)
    pub fn build(self) -> Result<ExportAccountResponseOauthGrantsItem, BuildError> {
        Ok(ExportAccountResponseOauthGrantsItem {
            api_key_id: self
                .api_key_id
                .ok_or_else(|| BuildError::missing_field("api_key_id"))?,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            client_name: self
                .client_name
                .ok_or_else(|| BuildError::missing_field("client_name"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
