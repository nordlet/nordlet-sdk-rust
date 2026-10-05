pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeesListHrRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: EmployeesListHrRequestFilterItemOp,
    pub value: EmployeesListHrRequestFilterItemValue,
}

impl EmployeesListHrRequestFilterItem {
    pub fn builder() -> EmployeesListHrRequestFilterItemBuilder {
        <EmployeesListHrRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesListHrRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<EmployeesListHrRequestFilterItemOp>,
    value: Option<EmployeesListHrRequestFilterItemValue>,
}

impl EmployeesListHrRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: EmployeesListHrRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: EmployeesListHrRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmployeesListHrRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](EmployeesListHrRequestFilterItemBuilder::field)
    /// - [`op`](EmployeesListHrRequestFilterItemBuilder::op)
    /// - [`value`](EmployeesListHrRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<EmployeesListHrRequestFilterItem, BuildError> {
        Ok(EmployeesListHrRequestFilterItem {
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
