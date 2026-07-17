pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1PosReportsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1PosReportsListRequestFilterItemOp,
    pub value: PostV1PosReportsListRequestFilterItemValue,
}

impl PostV1PosReportsListRequestFilterItem {
    pub fn builder() -> PostV1PosReportsListRequestFilterItemBuilder {
        <PostV1PosReportsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PosReportsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1PosReportsListRequestFilterItemOp>,
    value: Option<PostV1PosReportsListRequestFilterItemValue>,
}

impl PostV1PosReportsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1PosReportsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1PosReportsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PosReportsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PosReportsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1PosReportsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1PosReportsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1PosReportsListRequestFilterItem, BuildError> {
        Ok(PostV1PosReportsListRequestFilterItem {
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
