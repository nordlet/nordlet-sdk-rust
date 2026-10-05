pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeesRecordsListHrRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: EmployeesRecordsListHrRequestFilterItemOp,
    pub value: EmployeesRecordsListHrRequestFilterItemValue,
}

impl EmployeesRecordsListHrRequestFilterItem {
    pub fn builder() -> EmployeesRecordsListHrRequestFilterItemBuilder {
        <EmployeesRecordsListHrRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesRecordsListHrRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<EmployeesRecordsListHrRequestFilterItemOp>,
    value: Option<EmployeesRecordsListHrRequestFilterItemValue>,
}

impl EmployeesRecordsListHrRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: EmployeesRecordsListHrRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: EmployeesRecordsListHrRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmployeesRecordsListHrRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](EmployeesRecordsListHrRequestFilterItemBuilder::field)
    /// - [`op`](EmployeesRecordsListHrRequestFilterItemBuilder::op)
    /// - [`value`](EmployeesRecordsListHrRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<EmployeesRecordsListHrRequestFilterItem, BuildError> {
        Ok(EmployeesRecordsListHrRequestFilterItem {
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
