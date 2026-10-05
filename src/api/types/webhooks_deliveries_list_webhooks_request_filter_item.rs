pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeliveriesListWebhooksRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: DeliveriesListWebhooksRequestFilterItemOp,
    pub value: DeliveriesListWebhooksRequestFilterItemValue,
}

impl DeliveriesListWebhooksRequestFilterItem {
    pub fn builder() -> DeliveriesListWebhooksRequestFilterItemBuilder {
        <DeliveriesListWebhooksRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeliveriesListWebhooksRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<DeliveriesListWebhooksRequestFilterItemOp>,
    value: Option<DeliveriesListWebhooksRequestFilterItemValue>,
}

impl DeliveriesListWebhooksRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: DeliveriesListWebhooksRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: DeliveriesListWebhooksRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeliveriesListWebhooksRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](DeliveriesListWebhooksRequestFilterItemBuilder::field)
    /// - [`op`](DeliveriesListWebhooksRequestFilterItemBuilder::op)
    /// - [`value`](DeliveriesListWebhooksRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<DeliveriesListWebhooksRequestFilterItem, BuildError> {
        Ok(DeliveriesListWebhooksRequestFilterItem {
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
