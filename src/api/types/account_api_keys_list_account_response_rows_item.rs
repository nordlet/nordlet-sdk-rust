pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeysListAccountResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(rename = "lastUsedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_used_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "replacedByKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_by_key_id: Option<String>,
    #[serde(rename = "createdByUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
    #[serde(rename = "revokedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub revoked_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ApiKeysListAccountResponseRowsItem {
    pub fn builder() -> ApiKeysListAccountResponseRowsItemBuilder {
        <ApiKeysListAccountResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeysListAccountResponseRowsItemBuilder {
    id: Option<String>,
    name: Option<String>,
    scopes: Option<Vec<String>>,
    last_used_at: Option<DateTime<FixedOffset>>,
    expires_at: Option<DateTime<FixedOffset>>,
    replaced_by_key_id: Option<String>,
    created_by_user_id: Option<String>,
    revoked_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ApiKeysListAccountResponseRowsItemBuilder {
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

    pub fn last_used_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_used_at = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn replaced_by_key_id(mut self, value: impl Into<String>) -> Self {
        self.replaced_by_key_id = Some(value.into());
        self
    }

    pub fn created_by_user_id(mut self, value: impl Into<String>) -> Self {
        self.created_by_user_id = Some(value.into());
        self
    }

    pub fn revoked_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeysListAccountResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ApiKeysListAccountResponseRowsItemBuilder::id)
    /// - [`name`](ApiKeysListAccountResponseRowsItemBuilder::name)
    /// - [`scopes`](ApiKeysListAccountResponseRowsItemBuilder::scopes)
    /// - [`created_at`](ApiKeysListAccountResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<ApiKeysListAccountResponseRowsItem, BuildError> {
        Ok(ApiKeysListAccountResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
            last_used_at: self.last_used_at,
            expires_at: self.expires_at,
            replaced_by_key_id: self.replaced_by_key_id,
            created_by_user_id: self.created_by_user_id,
            revoked_at: self.revoked_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
