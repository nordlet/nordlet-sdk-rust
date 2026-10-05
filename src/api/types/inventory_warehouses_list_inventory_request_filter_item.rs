pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WarehousesListInventoryRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: WarehousesListInventoryRequestFilterItemOp,
    pub value: WarehousesListInventoryRequestFilterItemValue,
}

impl WarehousesListInventoryRequestFilterItem {
    pub fn builder() -> WarehousesListInventoryRequestFilterItemBuilder {
        <WarehousesListInventoryRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WarehousesListInventoryRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<WarehousesListInventoryRequestFilterItemOp>,
    value: Option<WarehousesListInventoryRequestFilterItemValue>,
}

impl WarehousesListInventoryRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: WarehousesListInventoryRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: WarehousesListInventoryRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WarehousesListInventoryRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](WarehousesListInventoryRequestFilterItemBuilder::field)
    /// - [`op`](WarehousesListInventoryRequestFilterItemBuilder::op)
    /// - [`value`](WarehousesListInventoryRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<WarehousesListInventoryRequestFilterItem, BuildError> {
        Ok(WarehousesListInventoryRequestFilterItem {
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
