pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1InventoryLandedCostsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1InventoryLandedCostsListRequestFilterItemOp,
    pub value: PostV1InventoryLandedCostsListRequestFilterItemValue,
}

impl PostV1InventoryLandedCostsListRequestFilterItem {
    pub fn builder() -> PostV1InventoryLandedCostsListRequestFilterItemBuilder {
        <PostV1InventoryLandedCostsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryLandedCostsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1InventoryLandedCostsListRequestFilterItemOp>,
    value: Option<PostV1InventoryLandedCostsListRequestFilterItemValue>,
}

impl PostV1InventoryLandedCostsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1InventoryLandedCostsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1InventoryLandedCostsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryLandedCostsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1InventoryLandedCostsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1InventoryLandedCostsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1InventoryLandedCostsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1InventoryLandedCostsListRequestFilterItem, BuildError> {
        Ok(PostV1InventoryLandedCostsListRequestFilterItem {
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
