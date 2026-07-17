pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1WebhooksDeliveriesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1WebhooksDeliveriesListRequestFilterItemOp,
    pub value: PostV1WebhooksDeliveriesListRequestFilterItemValue,
}

impl PostV1WebhooksDeliveriesListRequestFilterItem {
    pub fn builder() -> PostV1WebhooksDeliveriesListRequestFilterItemBuilder {
        <PostV1WebhooksDeliveriesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1WebhooksDeliveriesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1WebhooksDeliveriesListRequestFilterItemOp>,
    value: Option<PostV1WebhooksDeliveriesListRequestFilterItemValue>,
}

impl PostV1WebhooksDeliveriesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1WebhooksDeliveriesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1WebhooksDeliveriesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1WebhooksDeliveriesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1WebhooksDeliveriesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1WebhooksDeliveriesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1WebhooksDeliveriesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1WebhooksDeliveriesListRequestFilterItem, BuildError> {
        Ok(PostV1WebhooksDeliveriesListRequestFilterItem {
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
