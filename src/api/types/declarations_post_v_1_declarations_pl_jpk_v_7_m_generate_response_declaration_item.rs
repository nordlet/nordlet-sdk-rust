pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlJpkV7MGenerateResponseDeclarationItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1DeclarationsPlJpkV7MGenerateResponseDeclarationItem {
    pub fn builder() -> PostV1DeclarationsPlJpkV7MGenerateResponseDeclarationItemBuilder {
        <PostV1DeclarationsPlJpkV7MGenerateResponseDeclarationItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlJpkV7MGenerateResponseDeclarationItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl PostV1DeclarationsPlJpkV7MGenerateResponseDeclarationItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlJpkV7MGenerateResponseDeclarationItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1DeclarationsPlJpkV7MGenerateResponseDeclarationItemBuilder::field)
    /// - [`label`](PostV1DeclarationsPlJpkV7MGenerateResponseDeclarationItemBuilder::label)
    /// - [`value`](PostV1DeclarationsPlJpkV7MGenerateResponseDeclarationItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsPlJpkV7MGenerateResponseDeclarationItem, BuildError> {
        Ok(PostV1DeclarationsPlJpkV7MGenerateResponseDeclarationItem {
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
