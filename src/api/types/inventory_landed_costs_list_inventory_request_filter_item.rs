pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LandedCostsListInventoryRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: LandedCostsListInventoryRequestFilterItemOp,
    pub value: LandedCostsListInventoryRequestFilterItemValue,
}

impl LandedCostsListInventoryRequestFilterItem {
    pub fn builder() -> LandedCostsListInventoryRequestFilterItemBuilder {
        <LandedCostsListInventoryRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandedCostsListInventoryRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<LandedCostsListInventoryRequestFilterItemOp>,
    value: Option<LandedCostsListInventoryRequestFilterItemValue>,
}

impl LandedCostsListInventoryRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: LandedCostsListInventoryRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: LandedCostsListInventoryRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandedCostsListInventoryRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](LandedCostsListInventoryRequestFilterItemBuilder::field)
    /// - [`op`](LandedCostsListInventoryRequestFilterItemBuilder::op)
    /// - [`value`](LandedCostsListInventoryRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<LandedCostsListInventoryRequestFilterItem, BuildError> {
        Ok(LandedCostsListInventoryRequestFilterItem {
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
