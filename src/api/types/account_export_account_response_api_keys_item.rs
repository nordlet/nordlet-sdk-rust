pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportAccountResponseApiKeysItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "lastFour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_four: Option<String>,
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

impl ExportAccountResponseApiKeysItem {
    pub fn builder() -> ExportAccountResponseApiKeysItemBuilder {
        <ExportAccountResponseApiKeysItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportAccountResponseApiKeysItemBuilder {
    id: Option<String>,
    name: Option<String>,
    last_four: Option<String>,
    scopes: Option<Vec<String>>,
    last_used_at: Option<DateTime<FixedOffset>>,
    expires_at: Option<DateTime<FixedOffset>>,
    revoked_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ExportAccountResponseApiKeysItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn last_four(mut self, value: impl Into<String>) -> Self {
        self.last_four = Some(value.into());
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

    pub fn revoked_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportAccountResponseApiKeysItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExportAccountResponseApiKeysItemBuilder::id)
    /// - [`name`](ExportAccountResponseApiKeysItemBuilder::name)
    /// - [`scopes`](ExportAccountResponseApiKeysItemBuilder::scopes)
    /// - [`created_at`](ExportAccountResponseApiKeysItemBuilder::created_at)
    pub fn build(self) -> Result<ExportAccountResponseApiKeysItem, BuildError> {
        Ok(ExportAccountResponseApiKeysItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            last_four: self.last_four,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
            last_used_at: self.last_used_at,
            expires_at: self.expires_at,
            revoked_at: self.revoked_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
