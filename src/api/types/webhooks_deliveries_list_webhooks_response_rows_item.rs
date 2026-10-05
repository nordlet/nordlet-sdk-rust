pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeliveriesListWebhooksResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "subscriptionId")]
    #[serde(default)]
    pub subscription_id: String,
    #[serde(rename = "eventType")]
    #[serde(default)]
    pub event_type: String,
    pub status: DeliveriesListWebhooksResponseRowsItemStatus,
    #[serde(default)]
    pub attempts: i64,
    #[serde(rename = "lastError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "deliveredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub delivered_at: Option<DateTime<FixedOffset>>,
}

impl DeliveriesListWebhooksResponseRowsItem {
    pub fn builder() -> DeliveriesListWebhooksResponseRowsItemBuilder {
        <DeliveriesListWebhooksResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeliveriesListWebhooksResponseRowsItemBuilder {
    id: Option<String>,
    subscription_id: Option<String>,
    event_type: Option<String>,
    status: Option<DeliveriesListWebhooksResponseRowsItemStatus>,
    attempts: Option<i64>,
    last_error: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    delivered_at: Option<DateTime<FixedOffset>>,
}

impl DeliveriesListWebhooksResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    pub fn event_type(mut self, value: impl Into<String>) -> Self {
        self.event_type = Some(value.into());
        self
    }

    pub fn status(mut self, value: DeliveriesListWebhooksResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn last_error(mut self, value: impl Into<String>) -> Self {
        self.last_error = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn delivered_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.delivered_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeliveriesListWebhooksResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeliveriesListWebhooksResponseRowsItemBuilder::id)
    /// - [`subscription_id`](DeliveriesListWebhooksResponseRowsItemBuilder::subscription_id)
    /// - [`event_type`](DeliveriesListWebhooksResponseRowsItemBuilder::event_type)
    /// - [`status`](DeliveriesListWebhooksResponseRowsItemBuilder::status)
    /// - [`attempts`](DeliveriesListWebhooksResponseRowsItemBuilder::attempts)
    /// - [`created_at`](DeliveriesListWebhooksResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<DeliveriesListWebhooksResponseRowsItem, BuildError> {
        Ok(DeliveriesListWebhooksResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            subscription_id: self
                .subscription_id
                .ok_or_else(|| BuildError::missing_field("subscription_id"))?,
            event_type: self
                .event_type
                .ok_or_else(|| BuildError::missing_field("event_type"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            attempts: self
                .attempts
                .ok_or_else(|| BuildError::missing_field("attempts"))?,
            last_error: self.last_error,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            delivered_at: self.delivered_at,
        })
    }
}
