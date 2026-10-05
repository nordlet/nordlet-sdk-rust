pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesCreateHrRequestAttributesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl EmployeesCreateHrRequestAttributesItem {
    pub fn builder() -> EmployeesCreateHrRequestAttributesItemBuilder {
        <EmployeesCreateHrRequestAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesCreateHrRequestAttributesItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl EmployeesCreateHrRequestAttributesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmployeesCreateHrRequestAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](EmployeesCreateHrRequestAttributesItemBuilder::name)
    /// - [`value`](EmployeesCreateHrRequestAttributesItemBuilder::value)
    pub fn build(self) -> Result<EmployeesCreateHrRequestAttributesItem, BuildError> {
        Ok(EmployeesCreateHrRequestAttributesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
