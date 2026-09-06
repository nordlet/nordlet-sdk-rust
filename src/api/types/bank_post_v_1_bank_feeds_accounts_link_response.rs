pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsAccountsLinkResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "bankAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_id: Option<String>,
    #[serde(rename = "importTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_template_id: Option<String>,
    #[serde(rename = "syncSchedule")]
    pub sync_schedule: PostV1BankFeedsAccountsLinkResponseSyncSchedule,
    #[serde(rename = "externalId")]
    #[serde(default)]
    pub external_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(default)]
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "syncFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_from: Option<String>,
    #[serde(rename = "lastSyncedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synced_at: Option<String>,
}

impl PostV1BankFeedsAccountsLinkResponse {
    pub fn builder() -> PostV1BankFeedsAccountsLinkResponseBuilder {
        <PostV1BankFeedsAccountsLinkResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsAccountsLinkResponseBuilder {
    id: Option<String>,
    connection_id: Option<String>,
    bank_account_id: Option<String>,
    import_template_id: Option<String>,
    sync_schedule: Option<PostV1BankFeedsAccountsLinkResponseSyncSchedule>,
    external_id: Option<String>,
    iban: Option<String>,
    currency: Option<String>,
    name: Option<String>,
    product: Option<String>,
    sync_from: Option<String>,
    last_synced_at: Option<String>,
}

impl PostV1BankFeedsAccountsLinkResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn import_template_id(mut self, value: impl Into<String>) -> Self {
        self.import_template_id = Some(value.into());
        self
    }

    pub fn sync_schedule(mut self, value: PostV1BankFeedsAccountsLinkResponseSyncSchedule) -> Self {
        self.sync_schedule = Some(value);
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn iban(mut self, value: impl Into<String>) -> Self {
        self.iban = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn product(mut self, value: impl Into<String>) -> Self {
        self.product = Some(value.into());
        self
    }

    pub fn sync_from(mut self, value: impl Into<String>) -> Self {
        self.sync_from = Some(value.into());
        self
    }

    pub fn last_synced_at(mut self, value: impl Into<String>) -> Self {
        self.last_synced_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsAccountsLinkResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankFeedsAccountsLinkResponseBuilder::id)
    /// - [`connection_id`](PostV1BankFeedsAccountsLinkResponseBuilder::connection_id)
    /// - [`sync_schedule`](PostV1BankFeedsAccountsLinkResponseBuilder::sync_schedule)
    /// - [`external_id`](PostV1BankFeedsAccountsLinkResponseBuilder::external_id)
    /// - [`currency`](PostV1BankFeedsAccountsLinkResponseBuilder::currency)
    pub fn build(self) -> Result<PostV1BankFeedsAccountsLinkResponse, BuildError> {
        Ok(PostV1BankFeedsAccountsLinkResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            connection_id: self
                .connection_id
                .ok_or_else(|| BuildError::missing_field("connection_id"))?,
            bank_account_id: self.bank_account_id,
            import_template_id: self.import_template_id,
            sync_schedule: self
                .sync_schedule
                .ok_or_else(|| BuildError::missing_field("sync_schedule"))?,
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            iban: self.iban,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            name: self.name,
            product: self.product,
            sync_from: self.sync_from,
            last_synced_at: self.last_synced_at,
        })
    }
}
