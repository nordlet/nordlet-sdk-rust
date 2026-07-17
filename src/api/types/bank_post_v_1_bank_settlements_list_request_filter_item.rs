pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1BankSettlementsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1BankSettlementsListRequestFilterItemOp,
    pub value: PostV1BankSettlementsListRequestFilterItemValue,
}

impl PostV1BankSettlementsListRequestFilterItem {
    pub fn builder() -> PostV1BankSettlementsListRequestFilterItemBuilder {
        <PostV1BankSettlementsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankSettlementsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1BankSettlementsListRequestFilterItemOp>,
    value: Option<PostV1BankSettlementsListRequestFilterItemValue>,
}

impl PostV1BankSettlementsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1BankSettlementsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1BankSettlementsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankSettlementsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1BankSettlementsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1BankSettlementsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1BankSettlementsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1BankSettlementsListRequestFilterItem, BuildError> {
        Ok(PostV1BankSettlementsListRequestFilterItem {
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
