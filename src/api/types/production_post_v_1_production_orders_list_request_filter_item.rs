pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ProductionOrdersListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ProductionOrdersListRequestFilterItemOp,
    pub value: PostV1ProductionOrdersListRequestFilterItemValue,
}

impl PostV1ProductionOrdersListRequestFilterItem {
    pub fn builder() -> PostV1ProductionOrdersListRequestFilterItemBuilder {
        <PostV1ProductionOrdersListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionOrdersListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ProductionOrdersListRequestFilterItemOp>,
    value: Option<PostV1ProductionOrdersListRequestFilterItemValue>,
}

impl PostV1ProductionOrdersListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ProductionOrdersListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ProductionOrdersListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionOrdersListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ProductionOrdersListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ProductionOrdersListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ProductionOrdersListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ProductionOrdersListRequestFilterItem, BuildError> {
        Ok(PostV1ProductionOrdersListRequestFilterItem {
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
