pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehiclesListFleetRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: VehiclesListFleetRequestFilterItemOp,
    pub value: VehiclesListFleetRequestFilterItemValue,
}

impl VehiclesListFleetRequestFilterItem {
    pub fn builder() -> VehiclesListFleetRequestFilterItemBuilder {
        <VehiclesListFleetRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VehiclesListFleetRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<VehiclesListFleetRequestFilterItemOp>,
    value: Option<VehiclesListFleetRequestFilterItemValue>,
}

impl VehiclesListFleetRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: VehiclesListFleetRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: VehiclesListFleetRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VehiclesListFleetRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](VehiclesListFleetRequestFilterItemBuilder::field)
    /// - [`op`](VehiclesListFleetRequestFilterItemBuilder::op)
    /// - [`value`](VehiclesListFleetRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<VehiclesListFleetRequestFilterItem, BuildError> {
        Ok(VehiclesListFleetRequestFilterItem {
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
