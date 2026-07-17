pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsConfigsListResponseRowsItemFieldsItem {
    #[serde(default)]
    pub key: String,
    pub kind: PostV1DeclarationsConfigsListResponseRowsItemFieldsItemKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

impl PostV1DeclarationsConfigsListResponseRowsItemFieldsItem {
    pub fn builder() -> PostV1DeclarationsConfigsListResponseRowsItemFieldsItemBuilder {
        <PostV1DeclarationsConfigsListResponseRowsItemFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsConfigsListResponseRowsItemFieldsItemBuilder {
    key: Option<String>,
    kind: Option<PostV1DeclarationsConfigsListResponseRowsItemFieldsItemKind>,
    multiline: Option<bool>,
    options: Option<Vec<String>>,
}

impl PostV1DeclarationsConfigsListResponseRowsItemFieldsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn kind(
        mut self,
        value: PostV1DeclarationsConfigsListResponseRowsItemFieldsItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn multiline(mut self, value: bool) -> Self {
        self.multiline = Some(value);
        self
    }

    pub fn options(mut self, value: Vec<String>) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsConfigsListResponseRowsItemFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1DeclarationsConfigsListResponseRowsItemFieldsItemBuilder::key)
    /// - [`kind`](PostV1DeclarationsConfigsListResponseRowsItemFieldsItemBuilder::kind)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsConfigsListResponseRowsItemFieldsItem, BuildError> {
        Ok(PostV1DeclarationsConfigsListResponseRowsItemFieldsItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            multiline: self.multiline,
            options: self.options,
        })
    }
}
