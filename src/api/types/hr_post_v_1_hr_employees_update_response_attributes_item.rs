pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesUpdateResponseAttributesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1HrEmployeesUpdateResponseAttributesItem {
    pub fn builder() -> PostV1HrEmployeesUpdateResponseAttributesItemBuilder {
        <PostV1HrEmployeesUpdateResponseAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesUpdateResponseAttributesItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl PostV1HrEmployeesUpdateResponseAttributesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesUpdateResponseAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1HrEmployeesUpdateResponseAttributesItemBuilder::name)
    /// - [`value`](PostV1HrEmployeesUpdateResponseAttributesItemBuilder::value)
    pub fn build(self) -> Result<PostV1HrEmployeesUpdateResponseAttributesItem, BuildError> {
        Ok(PostV1HrEmployeesUpdateResponseAttributesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
