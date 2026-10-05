pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MaintenanceListProductionRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: MaintenanceListProductionRequestFilterItemOp,
    pub value: MaintenanceListProductionRequestFilterItemValue,
}

impl MaintenanceListProductionRequestFilterItem {
    pub fn builder() -> MaintenanceListProductionRequestFilterItemBuilder {
        <MaintenanceListProductionRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MaintenanceListProductionRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<MaintenanceListProductionRequestFilterItemOp>,
    value: Option<MaintenanceListProductionRequestFilterItemValue>,
}

impl MaintenanceListProductionRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: MaintenanceListProductionRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: MaintenanceListProductionRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MaintenanceListProductionRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](MaintenanceListProductionRequestFilterItemBuilder::field)
    /// - [`op`](MaintenanceListProductionRequestFilterItemBuilder::op)
    /// - [`value`](MaintenanceListProductionRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<MaintenanceListProductionRequestFilterItem, BuildError> {
        Ok(MaintenanceListProductionRequestFilterItem {
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
