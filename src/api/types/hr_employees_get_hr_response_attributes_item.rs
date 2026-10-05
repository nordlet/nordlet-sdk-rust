pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesGetHrResponseAttributesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl EmployeesGetHrResponseAttributesItem {
    pub fn builder() -> EmployeesGetHrResponseAttributesItemBuilder {
        <EmployeesGetHrResponseAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesGetHrResponseAttributesItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl EmployeesGetHrResponseAttributesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmployeesGetHrResponseAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](EmployeesGetHrResponseAttributesItemBuilder::name)
    /// - [`value`](EmployeesGetHrResponseAttributesItemBuilder::value)
    pub fn build(self) -> Result<EmployeesGetHrResponseAttributesItem, BuildError> {
        Ok(EmployeesGetHrResponseAttributesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
