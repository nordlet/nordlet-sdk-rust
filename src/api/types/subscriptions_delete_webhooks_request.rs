pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionsDeleteWebhooksRequest {
    #[serde(default)]
    pub id: String,
}

impl SubscriptionsDeleteWebhooksRequest {
    pub fn builder() -> SubscriptionsDeleteWebhooksRequestBuilder {
        <SubscriptionsDeleteWebhooksRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionsDeleteWebhooksRequestBuilder {
    id: Option<String>,
}

impl SubscriptionsDeleteWebhooksRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionsDeleteWebhooksRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SubscriptionsDeleteWebhooksRequestBuilder::id)
    pub fn build(self) -> Result<SubscriptionsDeleteWebhooksRequest, BuildError> {
        Ok(SubscriptionsDeleteWebhooksRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
