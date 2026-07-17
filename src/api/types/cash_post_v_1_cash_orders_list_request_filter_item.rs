pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1CashOrdersListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1CashOrdersListRequestFilterItemOp,
    pub value: PostV1CashOrdersListRequestFilterItemValue,
}

impl PostV1CashOrdersListRequestFilterItem {
    pub fn builder() -> PostV1CashOrdersListRequestFilterItemBuilder {
        <PostV1CashOrdersListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CashOrdersListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1CashOrdersListRequestFilterItemOp>,
    value: Option<PostV1CashOrdersListRequestFilterItemValue>,
}

impl PostV1CashOrdersListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1CashOrdersListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1CashOrdersListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CashOrdersListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1CashOrdersListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1CashOrdersListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1CashOrdersListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1CashOrdersListRequestFilterItem, BuildError> {
        Ok(PostV1CashOrdersListRequestFilterItem {
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
