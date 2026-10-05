pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FeedsConnectionsListBankResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub provider: String,
    #[serde(rename = "aspspName")]
    #[serde(default)]
    pub aspsp_name: String,
    #[serde(rename = "aspspCountry")]
    #[serde(default)]
    pub aspsp_country: String,
    #[serde(rename = "psuType")]
    pub psu_type: FeedsConnectionsListBankResponseRowsItemPsuType,
    pub status: FeedsConnectionsListBankResponseRowsItemStatus,
    #[serde(default)]
    pub reference: String,
    #[serde(rename = "consentExpiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub consent_expires_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "lastSyncedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_synced_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl FeedsConnectionsListBankResponseRowsItem {
    pub fn builder() -> FeedsConnectionsListBankResponseRowsItemBuilder {
        <FeedsConnectionsListBankResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsConnectionsListBankResponseRowsItemBuilder {
    id: Option<String>,
    provider: Option<String>,
    aspsp_name: Option<String>,
    aspsp_country: Option<String>,
    psu_type: Option<FeedsConnectionsListBankResponseRowsItemPsuType>,
    status: Option<FeedsConnectionsListBankResponseRowsItemStatus>,
    reference: Option<String>,
    consent_expires_at: Option<DateTime<FixedOffset>>,
    last_synced_at: Option<DateTime<FixedOffset>>,
    error: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl FeedsConnectionsListBankResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn aspsp_name(mut self, value: impl Into<String>) -> Self {
        self.aspsp_name = Some(value.into());
        self
    }

    pub fn aspsp_country(mut self, value: impl Into<String>) -> Self {
        self.aspsp_country = Some(value.into());
        self
    }

    pub fn psu_type(mut self, value: FeedsConnectionsListBankResponseRowsItemPsuType) -> Self {
        self.psu_type = Some(value);
        self
    }

    pub fn status(mut self, value: FeedsConnectionsListBankResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn consent_expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.consent_expires_at = Some(value);
        self
    }

    pub fn last_synced_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_synced_at = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedsConnectionsListBankResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](FeedsConnectionsListBankResponseRowsItemBuilder::id)
    /// - [`provider`](FeedsConnectionsListBankResponseRowsItemBuilder::provider)
    /// - [`aspsp_name`](FeedsConnectionsListBankResponseRowsItemBuilder::aspsp_name)
    /// - [`aspsp_country`](FeedsConnectionsListBankResponseRowsItemBuilder::aspsp_country)
    /// - [`psu_type`](FeedsConnectionsListBankResponseRowsItemBuilder::psu_type)
    /// - [`status`](FeedsConnectionsListBankResponseRowsItemBuilder::status)
    /// - [`reference`](FeedsConnectionsListBankResponseRowsItemBuilder::reference)
    /// - [`created_at`](FeedsConnectionsListBankResponseRowsItemBuilder::created_at)
    /// - [`updated_at`](FeedsConnectionsListBankResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<FeedsConnectionsListBankResponseRowsItem, BuildError> {
        Ok(FeedsConnectionsListBankResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            aspsp_name: self
                .aspsp_name
                .ok_or_else(|| BuildError::missing_field("aspsp_name"))?,
            aspsp_country: self
                .aspsp_country
                .ok_or_else(|| BuildError::missing_field("aspsp_country"))?,
            psu_type: self
                .psu_type
                .ok_or_else(|| BuildError::missing_field("psu_type"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
            consent_expires_at: self.consent_expires_at,
            last_synced_at: self.last_synced_at,
            error: self.error,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
