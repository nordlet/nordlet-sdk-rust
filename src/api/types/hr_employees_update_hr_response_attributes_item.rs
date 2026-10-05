pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesUpdateHrResponseAttributesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl EmployeesUpdateHrResponseAttributesItem {
    pub fn builder() -> EmployeesUpdateHrResponseAttributesItemBuilder {
        <EmployeesUpdateHrResponseAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesUpdateHrResponseAttributesItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl EmployeesUpdateHrResponseAttributesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmployeesUpdateHrResponseAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](EmployeesUpdateHrResponseAttributesItemBuilder::name)
    /// - [`value`](EmployeesUpdateHrResponseAttributesItemBuilder::value)
    pub fn build(self) -> Result<EmployeesUpdateHrResponseAttributesItem, BuildError> {
        Ok(EmployeesUpdateHrResponseAttributesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
