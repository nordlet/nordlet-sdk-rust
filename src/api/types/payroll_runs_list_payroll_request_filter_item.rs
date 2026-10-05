pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RunsListPayrollRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: RunsListPayrollRequestFilterItemOp,
    pub value: RunsListPayrollRequestFilterItemValue,
}

impl RunsListPayrollRequestFilterItem {
    pub fn builder() -> RunsListPayrollRequestFilterItemBuilder {
        <RunsListPayrollRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsListPayrollRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<RunsListPayrollRequestFilterItemOp>,
    value: Option<RunsListPayrollRequestFilterItemValue>,
}

impl RunsListPayrollRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: RunsListPayrollRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: RunsListPayrollRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunsListPayrollRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](RunsListPayrollRequestFilterItemBuilder::field)
    /// - [`op`](RunsListPayrollRequestFilterItemBuilder::op)
    /// - [`value`](RunsListPayrollRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<RunsListPayrollRequestFilterItem, BuildError> {
        Ok(RunsListPayrollRequestFilterItem {
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
