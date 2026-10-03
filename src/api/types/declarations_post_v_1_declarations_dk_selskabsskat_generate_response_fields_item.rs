pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDkSelskabsskatGenerateResponseFieldsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1DeclarationsDkSelskabsskatGenerateResponseFieldsItem {
    pub fn builder() -> PostV1DeclarationsDkSelskabsskatGenerateResponseFieldsItemBuilder {
        <PostV1DeclarationsDkSelskabsskatGenerateResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDkSelskabsskatGenerateResponseFieldsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl PostV1DeclarationsDkSelskabsskatGenerateResponseFieldsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsDkSelskabsskatGenerateResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1DeclarationsDkSelskabsskatGenerateResponseFieldsItemBuilder::field)
    /// - [`label`](PostV1DeclarationsDkSelskabsskatGenerateResponseFieldsItemBuilder::label)
    /// - [`value`](PostV1DeclarationsDkSelskabsskatGenerateResponseFieldsItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDkSelskabsskatGenerateResponseFieldsItem, BuildError> {
        Ok(PostV1DeclarationsDkSelskabsskatGenerateResponseFieldsItem {
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
