pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesListResponseRowsItemAttributesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1HrEmployeesListResponseRowsItemAttributesItem {
    pub fn builder() -> PostV1HrEmployeesListResponseRowsItemAttributesItemBuilder {
        <PostV1HrEmployeesListResponseRowsItemAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesListResponseRowsItemAttributesItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl PostV1HrEmployeesListResponseRowsItemAttributesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesListResponseRowsItemAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1HrEmployeesListResponseRowsItemAttributesItemBuilder::name)
    /// - [`value`](PostV1HrEmployeesListResponseRowsItemAttributesItemBuilder::value)
    pub fn build(self) -> Result<PostV1HrEmployeesListResponseRowsItemAttributesItem, BuildError> {
        Ok(PostV1HrEmployeesListResponseRowsItemAttributesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
