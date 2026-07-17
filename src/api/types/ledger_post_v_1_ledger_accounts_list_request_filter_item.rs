pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1LedgerAccountsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1LedgerAccountsListRequestFilterItemOp,
    pub value: PostV1LedgerAccountsListRequestFilterItemValue,
}

impl PostV1LedgerAccountsListRequestFilterItem {
    pub fn builder() -> PostV1LedgerAccountsListRequestFilterItemBuilder {
        <PostV1LedgerAccountsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1LedgerAccountsListRequestFilterItemOp>,
    value: Option<PostV1LedgerAccountsListRequestFilterItemValue>,
}

impl PostV1LedgerAccountsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1LedgerAccountsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1LedgerAccountsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1LedgerAccountsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1LedgerAccountsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1LedgerAccountsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1LedgerAccountsListRequestFilterItem, BuildError> {
        Ok(PostV1LedgerAccountsListRequestFilterItem {
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
