pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtPln204ComputeResponseLinesItem {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1DeclarationsLtPln204ComputeResponseLinesItem {
    pub fn builder() -> PostV1DeclarationsLtPln204ComputeResponseLinesItemBuilder {
        <PostV1DeclarationsLtPln204ComputeResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtPln204ComputeResponseLinesItemBuilder {
    key: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl PostV1DeclarationsLtPln204ComputeResponseLinesItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtPln204ComputeResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1DeclarationsLtPln204ComputeResponseLinesItemBuilder::key)
    /// - [`label`](PostV1DeclarationsLtPln204ComputeResponseLinesItemBuilder::label)
    /// - [`value`](PostV1DeclarationsLtPln204ComputeResponseLinesItemBuilder::value)
    pub fn build(self) -> Result<PostV1DeclarationsLtPln204ComputeResponseLinesItem, BuildError> {
        Ok(PostV1DeclarationsLtPln204ComputeResponseLinesItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
