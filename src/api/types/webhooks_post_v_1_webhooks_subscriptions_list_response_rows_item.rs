pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1WebhooksSubscriptionsListResponseRowsItem {
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

impl PostV1WebhooksSubscriptionsListResponseRowsItem {
    pub fn builder() -> PostV1WebhooksSubscriptionsListResponseRowsItemBuilder {
        <PostV1WebhooksSubscriptionsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1WebhooksSubscriptionsListResponseRowsItemBuilder {
    id: Option<String>,
    url: Option<String>,
    events: Option<Vec<String>>,
    is_active: Option<bool>,
    created_at: Option<String>,
}

impl PostV1WebhooksSubscriptionsListResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1WebhooksSubscriptionsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1WebhooksSubscriptionsListResponseRowsItemBuilder::id)
    /// - [`url`](PostV1WebhooksSubscriptionsListResponseRowsItemBuilder::url)
    /// - [`events`](PostV1WebhooksSubscriptionsListResponseRowsItemBuilder::events)
    /// - [`is_active`](PostV1WebhooksSubscriptionsListResponseRowsItemBuilder::is_active)
    /// - [`created_at`](PostV1WebhooksSubscriptionsListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1WebhooksSubscriptionsListResponseRowsItem, BuildError> {
        Ok(PostV1WebhooksSubscriptionsListResponseRowsItem {
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
