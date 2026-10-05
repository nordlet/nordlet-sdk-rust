pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LotsListInventoryRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: LotsListInventoryRequestFilterItemOp,
    pub value: LotsListInventoryRequestFilterItemValue,
}

impl LotsListInventoryRequestFilterItem {
    pub fn builder() -> LotsListInventoryRequestFilterItemBuilder {
        <LotsListInventoryRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LotsListInventoryRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<LotsListInventoryRequestFilterItemOp>,
    value: Option<LotsListInventoryRequestFilterItemValue>,
}

impl LotsListInventoryRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: LotsListInventoryRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: LotsListInventoryRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LotsListInventoryRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](LotsListInventoryRequestFilterItemBuilder::field)
    /// - [`op`](LotsListInventoryRequestFilterItemBuilder::op)
    /// - [`value`](LotsListInventoryRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<LotsListInventoryRequestFilterItem, BuildError> {
        Ok(LotsListInventoryRequestFilterItem {
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
