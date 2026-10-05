pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LandedCostsGetInventoryResponseLinesItem {
    #[serde(rename = "movementId")]
    #[serde(default)]
    pub movement_id: String,
    #[serde(rename = "allocatedAmount")]
    #[serde(default)]
    pub allocated_amount: String,
    #[serde(rename = "newUnitCost")]
    #[serde(default)]
    pub new_unit_cost: String,
}

impl LandedCostsGetInventoryResponseLinesItem {
    pub fn builder() -> LandedCostsGetInventoryResponseLinesItemBuilder {
        <LandedCostsGetInventoryResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandedCostsGetInventoryResponseLinesItemBuilder {
    movement_id: Option<String>,
    allocated_amount: Option<String>,
    new_unit_cost: Option<String>,
}

impl LandedCostsGetInventoryResponseLinesItemBuilder {
    pub fn movement_id(mut self, value: impl Into<String>) -> Self {
        self.movement_id = Some(value.into());
        self
    }

    pub fn allocated_amount(mut self, value: impl Into<String>) -> Self {
        self.allocated_amount = Some(value.into());
        self
    }

    pub fn new_unit_cost(mut self, value: impl Into<String>) -> Self {
        self.new_unit_cost = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LandedCostsGetInventoryResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`movement_id`](LandedCostsGetInventoryResponseLinesItemBuilder::movement_id)
    /// - [`allocated_amount`](LandedCostsGetInventoryResponseLinesItemBuilder::allocated_amount)
    /// - [`new_unit_cost`](LandedCostsGetInventoryResponseLinesItemBuilder::new_unit_cost)
    pub fn build(self) -> Result<LandedCostsGetInventoryResponseLinesItem, BuildError> {
        Ok(LandedCostsGetInventoryResponseLinesItem {
            movement_id: self
                .movement_id
                .ok_or_else(|| BuildError::missing_field("movement_id"))?,
            allocated_amount: self
                .allocated_amount
                .ok_or_else(|| BuildError::missing_field("allocated_amount"))?,
            new_unit_cost: self
                .new_unit_cost
                .ok_or_else(|| BuildError::missing_field("new_unit_cost"))?,
        })
    }
}
