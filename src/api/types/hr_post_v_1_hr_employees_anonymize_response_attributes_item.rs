pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesAnonymizeResponseAttributesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1HrEmployeesAnonymizeResponseAttributesItem {
    pub fn builder() -> PostV1HrEmployeesAnonymizeResponseAttributesItemBuilder {
        <PostV1HrEmployeesAnonymizeResponseAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesAnonymizeResponseAttributesItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl PostV1HrEmployeesAnonymizeResponseAttributesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesAnonymizeResponseAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1HrEmployeesAnonymizeResponseAttributesItemBuilder::name)
    /// - [`value`](PostV1HrEmployeesAnonymizeResponseAttributesItemBuilder::value)
    pub fn build(self) -> Result<PostV1HrEmployeesAnonymizeResponseAttributesItem, BuildError> {
        Ok(PostV1HrEmployeesAnonymizeResponseAttributesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
