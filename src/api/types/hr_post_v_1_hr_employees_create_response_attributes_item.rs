pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesCreateResponseAttributesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1HrEmployeesCreateResponseAttributesItem {
    pub fn builder() -> PostV1HrEmployeesCreateResponseAttributesItemBuilder {
        <PostV1HrEmployeesCreateResponseAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesCreateResponseAttributesItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl PostV1HrEmployeesCreateResponseAttributesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesCreateResponseAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1HrEmployeesCreateResponseAttributesItemBuilder::name)
    /// - [`value`](PostV1HrEmployeesCreateResponseAttributesItemBuilder::value)
    pub fn build(self) -> Result<PostV1HrEmployeesCreateResponseAttributesItem, BuildError> {
        Ok(PostV1HrEmployeesCreateResponseAttributesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
