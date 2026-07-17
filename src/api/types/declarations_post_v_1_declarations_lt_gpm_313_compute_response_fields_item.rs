pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtGpm313ComputeResponseFieldsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl PostV1DeclarationsLtGpm313ComputeResponseFieldsItem {
    pub fn builder() -> PostV1DeclarationsLtGpm313ComputeResponseFieldsItemBuilder {
        <PostV1DeclarationsLtGpm313ComputeResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtGpm313ComputeResponseFieldsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl PostV1DeclarationsLtGpm313ComputeResponseFieldsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtGpm313ComputeResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1DeclarationsLtGpm313ComputeResponseFieldsItemBuilder::field)
    /// - [`label`](PostV1DeclarationsLtGpm313ComputeResponseFieldsItemBuilder::label)
    /// - [`value`](PostV1DeclarationsLtGpm313ComputeResponseFieldsItemBuilder::value)
    pub fn build(self) -> Result<PostV1DeclarationsLtGpm313ComputeResponseFieldsItem, BuildError> {
        Ok(PostV1DeclarationsLtGpm313ComputeResponseFieldsItem {
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
