pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1SalesActsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1SalesActsListRequestFilterItemOp,
    pub value: PostV1SalesActsListRequestFilterItemValue,
}

impl PostV1SalesActsListRequestFilterItem {
    pub fn builder() -> PostV1SalesActsListRequestFilterItemBuilder {
        <PostV1SalesActsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesActsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1SalesActsListRequestFilterItemOp>,
    value: Option<PostV1SalesActsListRequestFilterItemValue>,
}

impl PostV1SalesActsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1SalesActsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1SalesActsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesActsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1SalesActsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1SalesActsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1SalesActsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1SalesActsListRequestFilterItem, BuildError> {
        Ok(PostV1SalesActsListRequestFilterItem {
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
