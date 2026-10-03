pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlCit8GenerateResponsePositionsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1DeclarationsPlCit8GenerateResponsePositionsItem {
    pub fn builder() -> PostV1DeclarationsPlCit8GenerateResponsePositionsItemBuilder {
        <PostV1DeclarationsPlCit8GenerateResponsePositionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlCit8GenerateResponsePositionsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl PostV1DeclarationsPlCit8GenerateResponsePositionsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlCit8GenerateResponsePositionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1DeclarationsPlCit8GenerateResponsePositionsItemBuilder::field)
    /// - [`label`](PostV1DeclarationsPlCit8GenerateResponsePositionsItemBuilder::label)
    /// - [`value`](PostV1DeclarationsPlCit8GenerateResponsePositionsItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsPlCit8GenerateResponsePositionsItem, BuildError> {
        Ok(PostV1DeclarationsPlCit8GenerateResponsePositionsItem {
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
