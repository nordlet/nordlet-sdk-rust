pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesRecordsListHrRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<EmployeesRecordsListHrRequestSortItemDir>,
}

impl EmployeesRecordsListHrRequestSortItem {
    pub fn builder() -> EmployeesRecordsListHrRequestSortItemBuilder {
        <EmployeesRecordsListHrRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesRecordsListHrRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<EmployeesRecordsListHrRequestSortItemDir>,
}

impl EmployeesRecordsListHrRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: EmployeesRecordsListHrRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmployeesRecordsListHrRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](EmployeesRecordsListHrRequestSortItemBuilder::field)
    pub fn build(self) -> Result<EmployeesRecordsListHrRequestSortItem, BuildError> {
        Ok(EmployeesRecordsListHrRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
