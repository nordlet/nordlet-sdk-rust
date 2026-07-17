pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1PayrollRunsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1PayrollRunsListRequestFilterItemOp,
    pub value: PostV1PayrollRunsListRequestFilterItemValue,
}

impl PostV1PayrollRunsListRequestFilterItem {
    pub fn builder() -> PostV1PayrollRunsListRequestFilterItemBuilder {
        <PostV1PayrollRunsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1PayrollRunsListRequestFilterItemOp>,
    value: Option<PostV1PayrollRunsListRequestFilterItemValue>,
}

impl PostV1PayrollRunsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1PayrollRunsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1PayrollRunsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PayrollRunsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1PayrollRunsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1PayrollRunsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1PayrollRunsListRequestFilterItem, BuildError> {
        Ok(PostV1PayrollRunsListRequestFilterItem {
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
