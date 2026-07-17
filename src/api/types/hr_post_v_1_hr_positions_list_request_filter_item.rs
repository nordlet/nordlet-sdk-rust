pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1HrPositionsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1HrPositionsListRequestFilterItemOp,
    pub value: PostV1HrPositionsListRequestFilterItemValue,
}

impl PostV1HrPositionsListRequestFilterItem {
    pub fn builder() -> PostV1HrPositionsListRequestFilterItemBuilder {
        <PostV1HrPositionsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrPositionsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1HrPositionsListRequestFilterItemOp>,
    value: Option<PostV1HrPositionsListRequestFilterItemValue>,
}

impl PostV1HrPositionsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1HrPositionsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1HrPositionsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrPositionsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1HrPositionsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1HrPositionsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1HrPositionsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1HrPositionsListRequestFilterItem, BuildError> {
        Ok(PostV1HrPositionsListRequestFilterItem {
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
