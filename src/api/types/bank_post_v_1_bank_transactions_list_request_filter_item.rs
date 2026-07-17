pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1BankTransactionsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1BankTransactionsListRequestFilterItemOp,
    pub value: PostV1BankTransactionsListRequestFilterItemValue,
}

impl PostV1BankTransactionsListRequestFilterItem {
    pub fn builder() -> PostV1BankTransactionsListRequestFilterItemBuilder {
        <PostV1BankTransactionsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankTransactionsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1BankTransactionsListRequestFilterItemOp>,
    value: Option<PostV1BankTransactionsListRequestFilterItemValue>,
}

impl PostV1BankTransactionsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1BankTransactionsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1BankTransactionsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankTransactionsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1BankTransactionsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1BankTransactionsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1BankTransactionsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1BankTransactionsListRequestFilterItem, BuildError> {
        Ok(PostV1BankTransactionsListRequestFilterItem {
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
