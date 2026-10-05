pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionsCreateWebhooksRequest {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub events: Vec<SubscriptionsCreateWebhooksRequestEventsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl SubscriptionsCreateWebhooksRequest {
    pub fn builder() -> SubscriptionsCreateWebhooksRequestBuilder {
        <SubscriptionsCreateWebhooksRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionsCreateWebhooksRequestBuilder {
    url: Option<String>,
    events: Option<Vec<SubscriptionsCreateWebhooksRequestEventsItem>>,
    secret: Option<String>,
}

impl SubscriptionsCreateWebhooksRequestBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<SubscriptionsCreateWebhooksRequestEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionsCreateWebhooksRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](SubscriptionsCreateWebhooksRequestBuilder::url)
    /// - [`events`](SubscriptionsCreateWebhooksRequestBuilder::events)
    pub fn build(self) -> Result<SubscriptionsCreateWebhooksRequest, BuildError> {
        Ok(SubscriptionsCreateWebhooksRequest {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            secret: self.secret,
        })
    }
}
