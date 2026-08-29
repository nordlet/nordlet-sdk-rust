pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsConnectionsCompleteResponse {
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
    pub psu_type: PostV1BankFeedsConnectionsCompleteResponsePsuType,
    pub status: PostV1BankFeedsConnectionsCompleteResponseStatus,
    #[serde(default)]
    pub reference: String,
    #[serde(rename = "consentExpiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_expires_at: Option<String>,
    #[serde(rename = "lastSyncedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synced_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub accounts: Vec<PostV1BankFeedsConnectionsCompleteResponseAccountsItem>,
}

impl PostV1BankFeedsConnectionsCompleteResponse {
    pub fn builder() -> PostV1BankFeedsConnectionsCompleteResponseBuilder {
        <PostV1BankFeedsConnectionsCompleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsConnectionsCompleteResponseBuilder {
    id: Option<String>,
    provider: Option<String>,
    aspsp_name: Option<String>,
    aspsp_country: Option<String>,
    psu_type: Option<PostV1BankFeedsConnectionsCompleteResponsePsuType>,
    status: Option<PostV1BankFeedsConnectionsCompleteResponseStatus>,
    reference: Option<String>,
    consent_expires_at: Option<String>,
    last_synced_at: Option<String>,
    error: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    accounts: Option<Vec<PostV1BankFeedsConnectionsCompleteResponseAccountsItem>>,
}

impl PostV1BankFeedsConnectionsCompleteResponseBuilder {
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

    pub fn psu_type(mut self, value: PostV1BankFeedsConnectionsCompleteResponsePsuType) -> Self {
        self.psu_type = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1BankFeedsConnectionsCompleteResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn consent_expires_at(mut self, value: impl Into<String>) -> Self {
        self.consent_expires_at = Some(value.into());
        self
    }

    pub fn last_synced_at(mut self, value: impl Into<String>) -> Self {
        self.last_synced_at = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn accounts(
        mut self,
        value: Vec<PostV1BankFeedsConnectionsCompleteResponseAccountsItem>,
    ) -> Self {
        self.accounts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsConnectionsCompleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankFeedsConnectionsCompleteResponseBuilder::id)
    /// - [`provider`](PostV1BankFeedsConnectionsCompleteResponseBuilder::provider)
    /// - [`aspsp_name`](PostV1BankFeedsConnectionsCompleteResponseBuilder::aspsp_name)
    /// - [`aspsp_country`](PostV1BankFeedsConnectionsCompleteResponseBuilder::aspsp_country)
    /// - [`psu_type`](PostV1BankFeedsConnectionsCompleteResponseBuilder::psu_type)
    /// - [`status`](PostV1BankFeedsConnectionsCompleteResponseBuilder::status)
    /// - [`reference`](PostV1BankFeedsConnectionsCompleteResponseBuilder::reference)
    /// - [`created_at`](PostV1BankFeedsConnectionsCompleteResponseBuilder::created_at)
    /// - [`updated_at`](PostV1BankFeedsConnectionsCompleteResponseBuilder::updated_at)
    /// - [`accounts`](PostV1BankFeedsConnectionsCompleteResponseBuilder::accounts)
    pub fn build(self) -> Result<PostV1BankFeedsConnectionsCompleteResponse, BuildError> {
        Ok(PostV1BankFeedsConnectionsCompleteResponse {
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
