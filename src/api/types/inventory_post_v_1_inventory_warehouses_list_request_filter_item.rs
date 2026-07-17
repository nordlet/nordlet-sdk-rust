pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1InventoryWarehousesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1InventoryWarehousesListRequestFilterItemOp,
    pub value: PostV1InventoryWarehousesListRequestFilterItemValue,
}

impl PostV1InventoryWarehousesListRequestFilterItem {
    pub fn builder() -> PostV1InventoryWarehousesListRequestFilterItemBuilder {
        <PostV1InventoryWarehousesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryWarehousesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1InventoryWarehousesListRequestFilterItemOp>,
    value: Option<PostV1InventoryWarehousesListRequestFilterItemValue>,
}

impl PostV1InventoryWarehousesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1InventoryWarehousesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1InventoryWarehousesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryWarehousesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1InventoryWarehousesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1InventoryWarehousesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1InventoryWarehousesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1InventoryWarehousesListRequestFilterItem, BuildError> {
        Ok(PostV1InventoryWarehousesListRequestFilterItem {
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
