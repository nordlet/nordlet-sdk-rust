pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1BankMandatesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1BankMandatesListRequestFilterItemOp,
    pub value: PostV1BankMandatesListRequestFilterItemValue,
}

impl PostV1BankMandatesListRequestFilterItem {
    pub fn builder() -> PostV1BankMandatesListRequestFilterItemBuilder {
        <PostV1BankMandatesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankMandatesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1BankMandatesListRequestFilterItemOp>,
    value: Option<PostV1BankMandatesListRequestFilterItemValue>,
}

impl PostV1BankMandatesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1BankMandatesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1BankMandatesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankMandatesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1BankMandatesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1BankMandatesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1BankMandatesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1BankMandatesListRequestFilterItem, BuildError> {
        Ok(PostV1BankMandatesListRequestFilterItem {
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
