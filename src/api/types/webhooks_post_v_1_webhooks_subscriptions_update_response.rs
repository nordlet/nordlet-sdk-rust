pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1WebhooksSubscriptionsUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1WebhooksSubscriptionsUpdateResponse {
    pub fn builder() -> PostV1WebhooksSubscriptionsUpdateResponseBuilder {
        <PostV1WebhooksSubscriptionsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1WebhooksSubscriptionsUpdateResponseBuilder {
    id: Option<String>,
    url: Option<String>,
    events: Option<Vec<String>>,
    is_active: Option<bool>,
    created_at: Option<String>,
}

impl PostV1WebhooksSubscriptionsUpdateResponseBuilder {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1WebhooksSubscriptionsUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1WebhooksSubscriptionsUpdateResponseBuilder::id)
    /// - [`url`](PostV1WebhooksSubscriptionsUpdateResponseBuilder::url)
    /// - [`events`](PostV1WebhooksSubscriptionsUpdateResponseBuilder::events)
    /// - [`is_active`](PostV1WebhooksSubscriptionsUpdateResponseBuilder::is_active)
    /// - [`created_at`](PostV1WebhooksSubscriptionsUpdateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1WebhooksSubscriptionsUpdateResponse, BuildError> {
        Ok(PostV1WebhooksSubscriptionsUpdateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
