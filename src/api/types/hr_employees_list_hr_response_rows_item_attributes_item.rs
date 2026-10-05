pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesListHrResponseRowsItemAttributesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl EmployeesListHrResponseRowsItemAttributesItem {
    pub fn builder() -> EmployeesListHrResponseRowsItemAttributesItemBuilder {
        <EmployeesListHrResponseRowsItemAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesListHrResponseRowsItemAttributesItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl EmployeesListHrResponseRowsItemAttributesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmployeesListHrResponseRowsItemAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](EmployeesListHrResponseRowsItemAttributesItemBuilder::name)
    /// - [`value`](EmployeesListHrResponseRowsItemAttributesItemBuilder::value)
    pub fn build(self) -> Result<EmployeesListHrResponseRowsItemAttributesItem, BuildError> {
        Ok(EmployeesListHrResponseRowsItemAttributesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
