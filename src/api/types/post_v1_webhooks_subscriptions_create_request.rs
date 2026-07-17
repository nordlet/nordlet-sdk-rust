pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1WebhooksSubscriptionsCreateRequest {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl PostV1WebhooksSubscriptionsCreateRequest {
    pub fn builder() -> PostV1WebhooksSubscriptionsCreateRequestBuilder {
        <PostV1WebhooksSubscriptionsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1WebhooksSubscriptionsCreateRequestBuilder {
    url: Option<String>,
    events: Option<Vec<String>>,
    secret: Option<String>,
}

impl PostV1WebhooksSubscriptionsCreateRequestBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<String>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1WebhooksSubscriptionsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](PostV1WebhooksSubscriptionsCreateRequestBuilder::url)
    /// - [`events`](PostV1WebhooksSubscriptionsCreateRequestBuilder::events)
    pub fn build(self) -> Result<PostV1WebhooksSubscriptionsCreateRequest, BuildError> {
        Ok(PostV1WebhooksSubscriptionsCreateRequest {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            secret: self.secret,
        })
    }
}
