pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1HrContractsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1HrContractsListRequestFilterItemOp,
    pub value: PostV1HrContractsListRequestFilterItemValue,
}

impl PostV1HrContractsListRequestFilterItem {
    pub fn builder() -> PostV1HrContractsListRequestFilterItemBuilder {
        <PostV1HrContractsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrContractsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1HrContractsListRequestFilterItemOp>,
    value: Option<PostV1HrContractsListRequestFilterItemValue>,
}

impl PostV1HrContractsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1HrContractsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1HrContractsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrContractsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1HrContractsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1HrContractsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1HrContractsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1HrContractsListRequestFilterItem, BuildError> {
        Ok(PostV1HrContractsListRequestFilterItem {
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
