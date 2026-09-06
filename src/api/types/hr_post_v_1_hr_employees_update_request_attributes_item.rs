pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesUpdateRequestAttributesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1HrEmployeesUpdateRequestAttributesItem {
    pub fn builder() -> PostV1HrEmployeesUpdateRequestAttributesItemBuilder {
        <PostV1HrEmployeesUpdateRequestAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesUpdateRequestAttributesItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl PostV1HrEmployeesUpdateRequestAttributesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesUpdateRequestAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1HrEmployeesUpdateRequestAttributesItemBuilder::name)
    /// - [`value`](PostV1HrEmployeesUpdateRequestAttributesItemBuilder::value)
    pub fn build(self) -> Result<PostV1HrEmployeesUpdateRequestAttributesItem, BuildError> {
        Ok(PostV1HrEmployeesUpdateRequestAttributesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
