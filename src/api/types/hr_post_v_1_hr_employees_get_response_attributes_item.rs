pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesGetResponseAttributesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1HrEmployeesGetResponseAttributesItem {
    pub fn builder() -> PostV1HrEmployeesGetResponseAttributesItemBuilder {
        <PostV1HrEmployeesGetResponseAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesGetResponseAttributesItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl PostV1HrEmployeesGetResponseAttributesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesGetResponseAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1HrEmployeesGetResponseAttributesItemBuilder::name)
    /// - [`value`](PostV1HrEmployeesGetResponseAttributesItemBuilder::value)
    pub fn build(self) -> Result<PostV1HrEmployeesGetResponseAttributesItem, BuildError> {
        Ok(PostV1HrEmployeesGetResponseAttributesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
