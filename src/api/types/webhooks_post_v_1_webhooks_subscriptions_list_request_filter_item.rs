pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1WebhooksSubscriptionsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1WebhooksSubscriptionsListRequestFilterItemOp,
    pub value: PostV1WebhooksSubscriptionsListRequestFilterItemValue,
}

impl PostV1WebhooksSubscriptionsListRequestFilterItem {
    pub fn builder() -> PostV1WebhooksSubscriptionsListRequestFilterItemBuilder {
        <PostV1WebhooksSubscriptionsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1WebhooksSubscriptionsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1WebhooksSubscriptionsListRequestFilterItemOp>,
    value: Option<PostV1WebhooksSubscriptionsListRequestFilterItemValue>,
}

impl PostV1WebhooksSubscriptionsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1WebhooksSubscriptionsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1WebhooksSubscriptionsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1WebhooksSubscriptionsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1WebhooksSubscriptionsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1WebhooksSubscriptionsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1WebhooksSubscriptionsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1WebhooksSubscriptionsListRequestFilterItem, BuildError> {
        Ok(PostV1WebhooksSubscriptionsListRequestFilterItem {
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
