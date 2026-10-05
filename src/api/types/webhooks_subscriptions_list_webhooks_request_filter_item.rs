pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscriptionsListWebhooksRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: SubscriptionsListWebhooksRequestFilterItemOp,
    pub value: SubscriptionsListWebhooksRequestFilterItemValue,
}

impl SubscriptionsListWebhooksRequestFilterItem {
    pub fn builder() -> SubscriptionsListWebhooksRequestFilterItemBuilder {
        <SubscriptionsListWebhooksRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionsListWebhooksRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<SubscriptionsListWebhooksRequestFilterItemOp>,
    value: Option<SubscriptionsListWebhooksRequestFilterItemValue>,
}

impl SubscriptionsListWebhooksRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: SubscriptionsListWebhooksRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: SubscriptionsListWebhooksRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionsListWebhooksRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](SubscriptionsListWebhooksRequestFilterItemBuilder::field)
    /// - [`op`](SubscriptionsListWebhooksRequestFilterItemBuilder::op)
    /// - [`value`](SubscriptionsListWebhooksRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<SubscriptionsListWebhooksRequestFilterItem, BuildError> {
        Ok(SubscriptionsListWebhooksRequestFilterItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            op: self.op.ok_or_else(|| BuildError::missing_field("op"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
