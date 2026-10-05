pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkCentersListProductionRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: WorkCentersListProductionRequestFilterItemOp,
    pub value: WorkCentersListProductionRequestFilterItemValue,
}

impl WorkCentersListProductionRequestFilterItem {
    pub fn builder() -> WorkCentersListProductionRequestFilterItemBuilder {
        <WorkCentersListProductionRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkCentersListProductionRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<WorkCentersListProductionRequestFilterItemOp>,
    value: Option<WorkCentersListProductionRequestFilterItemValue>,
}

impl WorkCentersListProductionRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: WorkCentersListProductionRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: WorkCentersListProductionRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkCentersListProductionRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](WorkCentersListProductionRequestFilterItemBuilder::field)
    /// - [`op`](WorkCentersListProductionRequestFilterItemBuilder::op)
    /// - [`value`](WorkCentersListProductionRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<WorkCentersListProductionRequestFilterItem, BuildError> {
        Ok(WorkCentersListProductionRequestFilterItem {
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
