pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FeedsConnectionsGetBankResponse {
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
    pub psu_type: FeedsConnectionsGetBankResponsePsuType,
    pub status: FeedsConnectionsGetBankResponseStatus,
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
    #[serde(default)]
    pub accounts: Vec<FeedsConnectionsGetBankResponseAccountsItem>,
}

impl FeedsConnectionsGetBankResponse {
    pub fn builder() -> FeedsConnectionsGetBankResponseBuilder {
        <FeedsConnectionsGetBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsConnectionsGetBankResponseBuilder {
    id: Option<String>,
    provider: Option<String>,
    aspsp_name: Option<String>,
    aspsp_country: Option<String>,
    psu_type: Option<FeedsConnectionsGetBankResponsePsuType>,
    status: Option<FeedsConnectionsGetBankResponseStatus>,
    reference: Option<String>,
    consent_expires_at: Option<DateTime<FixedOffset>>,
    last_synced_at: Option<DateTime<FixedOffset>>,
    error: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    accounts: Option<Vec<FeedsConnectionsGetBankResponseAccountsItem>>,
}

impl FeedsConnectionsGetBankResponseBuilder {
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

    pub fn psu_type(mut self, value: FeedsConnectionsGetBankResponsePsuType) -> Self {
        self.psu_type = Some(value);
        self
    }

    pub fn status(mut self, value: FeedsConnectionsGetBankResponseStatus) -> Self {
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

    pub fn accounts(mut self, value: Vec<FeedsConnectionsGetBankResponseAccountsItem>) -> Self {
        self.accounts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedsConnectionsGetBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](FeedsConnectionsGetBankResponseBuilder::id)
    /// - [`provider`](FeedsConnectionsGetBankResponseBuilder::provider)
    /// - [`aspsp_name`](FeedsConnectionsGetBankResponseBuilder::aspsp_name)
    /// - [`aspsp_country`](FeedsConnectionsGetBankResponseBuilder::aspsp_country)
    /// - [`psu_type`](FeedsConnectionsGetBankResponseBuilder::psu_type)
    /// - [`status`](FeedsConnectionsGetBankResponseBuilder::status)
    /// - [`reference`](FeedsConnectionsGetBankResponseBuilder::reference)
    /// - [`created_at`](FeedsConnectionsGetBankResponseBuilder::created_at)
    /// - [`updated_at`](FeedsConnectionsGetBankResponseBuilder::updated_at)
    /// - [`accounts`](FeedsConnectionsGetBankResponseBuilder::accounts)
    pub fn build(self) -> Result<FeedsConnectionsGetBankResponse, BuildError> {
        Ok(FeedsConnectionsGetBankResponse {
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
            accounts: self
                .accounts
                .ok_or_else(|| BuildError::missing_field("accounts"))?,
        })
    }
}
