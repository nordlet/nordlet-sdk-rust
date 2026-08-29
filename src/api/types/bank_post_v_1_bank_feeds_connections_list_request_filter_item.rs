pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1BankFeedsConnectionsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1BankFeedsConnectionsListRequestFilterItemOp,
    pub value: PostV1BankFeedsConnectionsListRequestFilterItemValue,
}

impl PostV1BankFeedsConnectionsListRequestFilterItem {
    pub fn builder() -> PostV1BankFeedsConnectionsListRequestFilterItemBuilder {
        <PostV1BankFeedsConnectionsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsConnectionsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1BankFeedsConnectionsListRequestFilterItemOp>,
    value: Option<PostV1BankFeedsConnectionsListRequestFilterItemValue>,
}

impl PostV1BankFeedsConnectionsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1BankFeedsConnectionsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1BankFeedsConnectionsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsConnectionsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1BankFeedsConnectionsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1BankFeedsConnectionsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1BankFeedsConnectionsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1BankFeedsConnectionsListRequestFilterItem, BuildError> {
        Ok(PostV1BankFeedsConnectionsListRequestFilterItem {
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
