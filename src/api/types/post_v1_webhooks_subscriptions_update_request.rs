pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1WebhooksSubscriptionsUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl PostV1WebhooksSubscriptionsUpdateRequest {
    pub fn builder() -> PostV1WebhooksSubscriptionsUpdateRequestBuilder {
        <PostV1WebhooksSubscriptionsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1WebhooksSubscriptionsUpdateRequestBuilder {
    id: Option<String>,
    url: Option<String>,
    events: Option<Vec<String>>,
    is_active: Option<bool>,
}

impl PostV1WebhooksSubscriptionsUpdateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1WebhooksSubscriptionsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1WebhooksSubscriptionsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1WebhooksSubscriptionsUpdateRequest, BuildError> {
        Ok(PostV1WebhooksSubscriptionsUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url,
            events: self.events,
            is_active: self.is_active,
        })
    }
}
