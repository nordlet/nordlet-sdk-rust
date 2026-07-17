pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1LedgerOwnersListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1LedgerOwnersListRequestFilterItemOp,
    pub value: PostV1LedgerOwnersListRequestFilterItemValue,
}

impl PostV1LedgerOwnersListRequestFilterItem {
    pub fn builder() -> PostV1LedgerOwnersListRequestFilterItemBuilder {
        <PostV1LedgerOwnersListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerOwnersListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1LedgerOwnersListRequestFilterItemOp>,
    value: Option<PostV1LedgerOwnersListRequestFilterItemValue>,
}

impl PostV1LedgerOwnersListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1LedgerOwnersListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1LedgerOwnersListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerOwnersListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1LedgerOwnersListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1LedgerOwnersListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1LedgerOwnersListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1LedgerOwnersListRequestFilterItem, BuildError> {
        Ok(PostV1LedgerOwnersListRequestFilterItem {
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
