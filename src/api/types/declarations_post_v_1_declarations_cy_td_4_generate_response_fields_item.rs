pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsCyTd4GenerateResponseFieldsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1DeclarationsCyTd4GenerateResponseFieldsItem {
    pub fn builder() -> PostV1DeclarationsCyTd4GenerateResponseFieldsItemBuilder {
        <PostV1DeclarationsCyTd4GenerateResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsCyTd4GenerateResponseFieldsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl PostV1DeclarationsCyTd4GenerateResponseFieldsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsCyTd4GenerateResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1DeclarationsCyTd4GenerateResponseFieldsItemBuilder::field)
    /// - [`label`](PostV1DeclarationsCyTd4GenerateResponseFieldsItemBuilder::label)
    /// - [`value`](PostV1DeclarationsCyTd4GenerateResponseFieldsItemBuilder::value)
    pub fn build(self) -> Result<PostV1DeclarationsCyTd4GenerateResponseFieldsItem, BuildError> {
        Ok(PostV1DeclarationsCyTd4GenerateResponseFieldsItem {
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
