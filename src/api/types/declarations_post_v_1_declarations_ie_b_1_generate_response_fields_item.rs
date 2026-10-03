pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsIeB1GenerateResponseFieldsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1DeclarationsIeB1GenerateResponseFieldsItem {
    pub fn builder() -> PostV1DeclarationsIeB1GenerateResponseFieldsItemBuilder {
        <PostV1DeclarationsIeB1GenerateResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsIeB1GenerateResponseFieldsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl PostV1DeclarationsIeB1GenerateResponseFieldsItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsIeB1GenerateResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1DeclarationsIeB1GenerateResponseFieldsItemBuilder::field)
    /// - [`label`](PostV1DeclarationsIeB1GenerateResponseFieldsItemBuilder::label)
    /// - [`value`](PostV1DeclarationsIeB1GenerateResponseFieldsItemBuilder::value)
    pub fn build(self) -> Result<PostV1DeclarationsIeB1GenerateResponseFieldsItem, BuildError> {
        Ok(PostV1DeclarationsIeB1GenerateResponseFieldsItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
