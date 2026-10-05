pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesCreateHrResponseAttributesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl EmployeesCreateHrResponseAttributesItem {
    pub fn builder() -> EmployeesCreateHrResponseAttributesItemBuilder {
        <EmployeesCreateHrResponseAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesCreateHrResponseAttributesItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl EmployeesCreateHrResponseAttributesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmployeesCreateHrResponseAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](EmployeesCreateHrResponseAttributesItemBuilder::name)
    /// - [`value`](EmployeesCreateHrResponseAttributesItemBuilder::value)
    pub fn build(self) -> Result<EmployeesCreateHrResponseAttributesItem, BuildError> {
        Ok(EmployeesCreateHrResponseAttributesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
