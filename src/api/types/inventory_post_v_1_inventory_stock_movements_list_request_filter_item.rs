pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1InventoryStockMovementsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1InventoryStockMovementsListRequestFilterItemOp,
    pub value: PostV1InventoryStockMovementsListRequestFilterItemValue,
}

impl PostV1InventoryStockMovementsListRequestFilterItem {
    pub fn builder() -> PostV1InventoryStockMovementsListRequestFilterItemBuilder {
        <PostV1InventoryStockMovementsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryStockMovementsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1InventoryStockMovementsListRequestFilterItemOp>,
    value: Option<PostV1InventoryStockMovementsListRequestFilterItemValue>,
}

impl PostV1InventoryStockMovementsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1InventoryStockMovementsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1InventoryStockMovementsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryStockMovementsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1InventoryStockMovementsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1InventoryStockMovementsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1InventoryStockMovementsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1InventoryStockMovementsListRequestFilterItem, BuildError> {
        Ok(PostV1InventoryStockMovementsListRequestFilterItem {
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
