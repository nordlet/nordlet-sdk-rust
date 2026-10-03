pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsIeCt1GenerateResponseFieldsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1DeclarationsIeCt1GenerateResponseFieldsItem {
    pub fn builder() -> PostV1DeclarationsIeCt1GenerateResponseFieldsItemBuilder {
        <PostV1DeclarationsIeCt1GenerateResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsIeCt1GenerateResponseFieldsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl PostV1DeclarationsIeCt1GenerateResponseFieldsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsIeCt1GenerateResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1DeclarationsIeCt1GenerateResponseFieldsItemBuilder::field)
    /// - [`label`](PostV1DeclarationsIeCt1GenerateResponseFieldsItemBuilder::label)
    /// - [`value`](PostV1DeclarationsIeCt1GenerateResponseFieldsItemBuilder::value)
    pub fn build(self) -> Result<PostV1DeclarationsIeCt1GenerateResponseFieldsItem, BuildError> {
        Ok(PostV1DeclarationsIeCt1GenerateResponseFieldsItem {
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
