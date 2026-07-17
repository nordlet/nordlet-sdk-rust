pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1BankAccountsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1BankAccountsListRequestFilterItemOp,
    pub value: PostV1BankAccountsListRequestFilterItemValue,
}

impl PostV1BankAccountsListRequestFilterItem {
    pub fn builder() -> PostV1BankAccountsListRequestFilterItemBuilder {
        <PostV1BankAccountsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankAccountsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1BankAccountsListRequestFilterItemOp>,
    value: Option<PostV1BankAccountsListRequestFilterItemValue>,
}

impl PostV1BankAccountsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1BankAccountsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1BankAccountsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankAccountsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1BankAccountsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1BankAccountsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1BankAccountsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1BankAccountsListRequestFilterItem, BuildError> {
        Ok(PostV1BankAccountsListRequestFilterItem {
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
