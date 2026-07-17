pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollDepartmentsListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

impl PostV1PayrollDepartmentsListResponseRowsItem {
    pub fn builder() -> PostV1PayrollDepartmentsListResponseRowsItemBuilder {
        <PostV1PayrollDepartmentsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollDepartmentsListResponseRowsItemBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
}

impl PostV1PayrollDepartmentsListResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1PayrollDepartmentsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PayrollDepartmentsListResponseRowsItemBuilder::id)
    /// - [`code`](PostV1PayrollDepartmentsListResponseRowsItemBuilder::code)
    /// - [`name`](PostV1PayrollDepartmentsListResponseRowsItemBuilder::name)
    pub fn build(self) -> Result<PostV1PayrollDepartmentsListResponseRowsItem, BuildError> {
        Ok(PostV1PayrollDepartmentsListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
