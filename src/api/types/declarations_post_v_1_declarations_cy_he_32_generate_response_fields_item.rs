pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsCyHe32GenerateResponseFieldsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1DeclarationsCyHe32GenerateResponseFieldsItem {
    pub fn builder() -> PostV1DeclarationsCyHe32GenerateResponseFieldsItemBuilder {
        <PostV1DeclarationsCyHe32GenerateResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsCyHe32GenerateResponseFieldsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl PostV1DeclarationsCyHe32GenerateResponseFieldsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsCyHe32GenerateResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1DeclarationsCyHe32GenerateResponseFieldsItemBuilder::field)
    /// - [`label`](PostV1DeclarationsCyHe32GenerateResponseFieldsItemBuilder::label)
    /// - [`value`](PostV1DeclarationsCyHe32GenerateResponseFieldsItemBuilder::value)
    pub fn build(self) -> Result<PostV1DeclarationsCyHe32GenerateResponseFieldsItem, BuildError> {
        Ok(PostV1DeclarationsCyHe32GenerateResponseFieldsItem {
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
