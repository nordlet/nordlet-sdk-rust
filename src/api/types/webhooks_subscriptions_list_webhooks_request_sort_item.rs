pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionsListWebhooksRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<SubscriptionsListWebhooksRequestSortItemDir>,
}

impl SubscriptionsListWebhooksRequestSortItem {
    pub fn builder() -> SubscriptionsListWebhooksRequestSortItemBuilder {
        <SubscriptionsListWebhooksRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionsListWebhooksRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<SubscriptionsListWebhooksRequestSortItemDir>,
}

impl SubscriptionsListWebhooksRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: SubscriptionsListWebhooksRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionsListWebhooksRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](SubscriptionsListWebhooksRequestSortItemBuilder::field)
    pub fn build(self) -> Result<SubscriptionsListWebhooksRequestSortItem, BuildError> {
        Ok(SubscriptionsListWebhooksRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
