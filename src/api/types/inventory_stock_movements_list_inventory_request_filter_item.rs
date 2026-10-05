pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StockMovementsListInventoryRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: StockMovementsListInventoryRequestFilterItemOp,
    pub value: StockMovementsListInventoryRequestFilterItemValue,
}

impl StockMovementsListInventoryRequestFilterItem {
    pub fn builder() -> StockMovementsListInventoryRequestFilterItemBuilder {
        <StockMovementsListInventoryRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockMovementsListInventoryRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<StockMovementsListInventoryRequestFilterItemOp>,
    value: Option<StockMovementsListInventoryRequestFilterItemValue>,
}

impl StockMovementsListInventoryRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: StockMovementsListInventoryRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: StockMovementsListInventoryRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StockMovementsListInventoryRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](StockMovementsListInventoryRequestFilterItemBuilder::field)
    /// - [`op`](StockMovementsListInventoryRequestFilterItemBuilder::op)
    /// - [`value`](StockMovementsListInventoryRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<StockMovementsListInventoryRequestFilterItem, BuildError> {
        Ok(StockMovementsListInventoryRequestFilterItem {
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
