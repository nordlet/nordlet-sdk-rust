pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1InventoryLotsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1InventoryLotsListRequestFilterItemOp,
    pub value: PostV1InventoryLotsListRequestFilterItemValue,
}

impl PostV1InventoryLotsListRequestFilterItem {
    pub fn builder() -> PostV1InventoryLotsListRequestFilterItemBuilder {
        <PostV1InventoryLotsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryLotsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1InventoryLotsListRequestFilterItemOp>,
    value: Option<PostV1InventoryLotsListRequestFilterItemValue>,
}

impl PostV1InventoryLotsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1InventoryLotsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1InventoryLotsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryLotsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1InventoryLotsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1InventoryLotsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1InventoryLotsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1InventoryLotsListRequestFilterItem, BuildError> {
        Ok(PostV1InventoryLotsListRequestFilterItem {
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
