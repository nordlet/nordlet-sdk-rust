pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DepartmentsListPayrollResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

impl DepartmentsListPayrollResponseRowsItem {
    pub fn builder() -> DepartmentsListPayrollResponseRowsItemBuilder {
        <DepartmentsListPayrollResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DepartmentsListPayrollResponseRowsItemBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
}

impl DepartmentsListPayrollResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DepartmentsListPayrollResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DepartmentsListPayrollResponseRowsItemBuilder::id)
    /// - [`code`](DepartmentsListPayrollResponseRowsItemBuilder::code)
    /// - [`name`](DepartmentsListPayrollResponseRowsItemBuilder::name)
    pub fn build(self) -> Result<DepartmentsListPayrollResponseRowsItem, BuildError> {
        Ok(DepartmentsListPayrollResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
