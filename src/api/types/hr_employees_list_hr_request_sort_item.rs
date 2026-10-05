pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesListHrRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<EmployeesListHrRequestSortItemDir>,
}

impl EmployeesListHrRequestSortItem {
    pub fn builder() -> EmployeesListHrRequestSortItemBuilder {
        <EmployeesListHrRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesListHrRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<EmployeesListHrRequestSortItemDir>,
}

impl EmployeesListHrRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: EmployeesListHrRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmployeesListHrRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](EmployeesListHrRequestSortItemBuilder::field)
    pub fn build(self) -> Result<EmployeesListHrRequestSortItem, BuildError> {
        Ok(EmployeesListHrRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
