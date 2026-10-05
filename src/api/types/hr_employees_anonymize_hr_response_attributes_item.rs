pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesAnonymizeHrResponseAttributesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl EmployeesAnonymizeHrResponseAttributesItem {
    pub fn builder() -> EmployeesAnonymizeHrResponseAttributesItemBuilder {
        <EmployeesAnonymizeHrResponseAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesAnonymizeHrResponseAttributesItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl EmployeesAnonymizeHrResponseAttributesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmployeesAnonymizeHrResponseAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](EmployeesAnonymizeHrResponseAttributesItemBuilder::name)
    /// - [`value`](EmployeesAnonymizeHrResponseAttributesItemBuilder::value)
    pub fn build(self) -> Result<EmployeesAnonymizeHrResponseAttributesItem, BuildError> {
        Ok(EmployeesAnonymizeHrResponseAttributesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
