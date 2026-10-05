pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionsUpdateWebhooksResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "consecutiveFailures")]
    #[serde(default)]
    pub consecutive_failures: i64,
    #[serde(rename = "lastDeliveryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_delivery_status: Option<SubscriptionsUpdateWebhooksResponseLastDeliveryStatus>,
    #[serde(rename = "lastDeliveryAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_delivery_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "pausedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub paused_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl SubscriptionsUpdateWebhooksResponse {
    pub fn builder() -> SubscriptionsUpdateWebhooksResponseBuilder {
        <SubscriptionsUpdateWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionsUpdateWebhooksResponseBuilder {
    id: Option<String>,
    url: Option<String>,
    events: Option<Vec<String>>,
    is_active: Option<bool>,
    consecutive_failures: Option<i64>,
    last_delivery_status: Option<SubscriptionsUpdateWebhooksResponseLastDeliveryStatus>,
    last_delivery_at: Option<DateTime<FixedOffset>>,
    paused_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl SubscriptionsUpdateWebhooksResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<String>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn consecutive_failures(mut self, value: i64) -> Self {
        self.consecutive_failures = Some(value);
        self
    }

    pub fn last_delivery_status(
        mut self,
        value: SubscriptionsUpdateWebhooksResponseLastDeliveryStatus,
    ) -> Self {
        self.last_delivery_status = Some(value);
        self
    }

    pub fn last_delivery_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_delivery_at = Some(value);
        self
    }

    pub fn paused_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.paused_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionsUpdateWebhooksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SubscriptionsUpdateWebhooksResponseBuilder::id)
    /// - [`url`](SubscriptionsUpdateWebhooksResponseBuilder::url)
    /// - [`events`](SubscriptionsUpdateWebhooksResponseBuilder::events)
    /// - [`is_active`](SubscriptionsUpdateWebhooksResponseBuilder::is_active)
    /// - [`consecutive_failures`](SubscriptionsUpdateWebhooksResponseBuilder::consecutive_failures)
    /// - [`created_at`](SubscriptionsUpdateWebhooksResponseBuilder::created_at)
    pub fn build(self) -> Result<SubscriptionsUpdateWebhooksResponse, BuildError> {
        Ok(SubscriptionsUpdateWebhooksResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            consecutive_failures: self
                .consecutive_failures
                .ok_or_else(|| BuildError::missing_field("consecutive_failures"))?,
            last_delivery_status: self.last_delivery_status,
            last_delivery_at: self.last_delivery_at,
            paused_at: self.paused_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
