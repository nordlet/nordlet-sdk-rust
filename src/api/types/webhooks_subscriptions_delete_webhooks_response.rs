pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionsDeleteWebhooksResponse {
    #[serde(default)]
    pub id: String,
}

impl SubscriptionsDeleteWebhooksResponse {
    pub fn builder() -> SubscriptionsDeleteWebhooksResponseBuilder {
        <SubscriptionsDeleteWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionsDeleteWebhooksResponseBuilder {
    id: Option<String>,
}

impl SubscriptionsDeleteWebhooksResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionsDeleteWebhooksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SubscriptionsDeleteWebhooksResponseBuilder::id)
    pub fn build(self) -> Result<SubscriptionsDeleteWebhooksResponse, BuildError> {
        Ok(SubscriptionsDeleteWebhooksResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
