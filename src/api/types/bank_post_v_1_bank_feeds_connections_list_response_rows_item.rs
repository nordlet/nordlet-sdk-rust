pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsConnectionsListResponseRowsItem {
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
    pub psu_type: PostV1BankFeedsConnectionsListResponseRowsItemPsuType,
    pub status: PostV1BankFeedsConnectionsListResponseRowsItemStatus,
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
}

impl PostV1BankFeedsConnectionsListResponseRowsItem {
    pub fn builder() -> PostV1BankFeedsConnectionsListResponseRowsItemBuilder {
        <PostV1BankFeedsConnectionsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsConnectionsListResponseRowsItemBuilder {
    id: Option<String>,
    provider: Option<String>,
    aspsp_name: Option<String>,
    aspsp_country: Option<String>,
    psu_type: Option<PostV1BankFeedsConnectionsListResponseRowsItemPsuType>,
    status: Option<PostV1BankFeedsConnectionsListResponseRowsItemStatus>,
    reference: Option<String>,
    consent_expires_at: Option<String>,
    last_synced_at: Option<String>,
    error: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1BankFeedsConnectionsListResponseRowsItemBuilder {
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

    pub fn psu_type(
        mut self,
        value: PostV1BankFeedsConnectionsListResponseRowsItemPsuType,
    ) -> Self {
        self.psu_type = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1BankFeedsConnectionsListResponseRowsItemStatus) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1BankFeedsConnectionsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankFeedsConnectionsListResponseRowsItemBuilder::id)
    /// - [`provider`](PostV1BankFeedsConnectionsListResponseRowsItemBuilder::provider)
    /// - [`aspsp_name`](PostV1BankFeedsConnectionsListResponseRowsItemBuilder::aspsp_name)
    /// - [`aspsp_country`](PostV1BankFeedsConnectionsListResponseRowsItemBuilder::aspsp_country)
    /// - [`psu_type`](PostV1BankFeedsConnectionsListResponseRowsItemBuilder::psu_type)
    /// - [`status`](PostV1BankFeedsConnectionsListResponseRowsItemBuilder::status)
    /// - [`reference`](PostV1BankFeedsConnectionsListResponseRowsItemBuilder::reference)
    /// - [`created_at`](PostV1BankFeedsConnectionsListResponseRowsItemBuilder::created_at)
    /// - [`updated_at`](PostV1BankFeedsConnectionsListResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<PostV1BankFeedsConnectionsListResponseRowsItem, BuildError> {
        Ok(PostV1BankFeedsConnectionsListResponseRowsItem {
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
