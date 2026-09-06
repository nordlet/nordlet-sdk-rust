pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesCreateRequestAttributesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1HrEmployeesCreateRequestAttributesItem {
    pub fn builder() -> PostV1HrEmployeesCreateRequestAttributesItemBuilder {
        <PostV1HrEmployeesCreateRequestAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesCreateRequestAttributesItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl PostV1HrEmployeesCreateRequestAttributesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesCreateRequestAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1HrEmployeesCreateRequestAttributesItemBuilder::name)
    /// - [`value`](PostV1HrEmployeesCreateRequestAttributesItemBuilder::value)
    pub fn build(self) -> Result<PostV1HrEmployeesCreateRequestAttributesItem, BuildError> {
        Ok(PostV1HrEmployeesCreateRequestAttributesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
