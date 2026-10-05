pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssignmentsListFleetRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: AssignmentsListFleetRequestFilterItemOp,
    pub value: AssignmentsListFleetRequestFilterItemValue,
}

impl AssignmentsListFleetRequestFilterItem {
    pub fn builder() -> AssignmentsListFleetRequestFilterItemBuilder {
        <AssignmentsListFleetRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssignmentsListFleetRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<AssignmentsListFleetRequestFilterItemOp>,
    value: Option<AssignmentsListFleetRequestFilterItemValue>,
}

impl AssignmentsListFleetRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: AssignmentsListFleetRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: AssignmentsListFleetRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssignmentsListFleetRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](AssignmentsListFleetRequestFilterItemBuilder::field)
    /// - [`op`](AssignmentsListFleetRequestFilterItemBuilder::op)
    /// - [`value`](AssignmentsListFleetRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<AssignmentsListFleetRequestFilterItem, BuildError> {
        Ok(AssignmentsListFleetRequestFilterItem {
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
