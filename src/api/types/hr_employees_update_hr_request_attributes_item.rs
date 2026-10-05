pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesUpdateHrRequestAttributesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl EmployeesUpdateHrRequestAttributesItem {
    pub fn builder() -> EmployeesUpdateHrRequestAttributesItemBuilder {
        <EmployeesUpdateHrRequestAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesUpdateHrRequestAttributesItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl EmployeesUpdateHrRequestAttributesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmployeesUpdateHrRequestAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](EmployeesUpdateHrRequestAttributesItemBuilder::name)
    /// - [`value`](EmployeesUpdateHrRequestAttributesItemBuilder::value)
    pub fn build(self) -> Result<EmployeesUpdateHrRequestAttributesItem, BuildError> {
        Ok(EmployeesUpdateHrRequestAttributesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
