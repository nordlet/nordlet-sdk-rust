pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsConfigsUpdateResponseFieldsItem {
    #[serde(default)]
    pub key: String,
    pub kind: PostV1DeclarationsConfigsUpdateResponseFieldsItemKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

impl PostV1DeclarationsConfigsUpdateResponseFieldsItem {
    pub fn builder() -> PostV1DeclarationsConfigsUpdateResponseFieldsItemBuilder {
        <PostV1DeclarationsConfigsUpdateResponseFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsConfigsUpdateResponseFieldsItemBuilder {
    key: Option<String>,
    kind: Option<PostV1DeclarationsConfigsUpdateResponseFieldsItemKind>,
    multiline: Option<bool>,
    options: Option<Vec<String>>,
}

impl PostV1DeclarationsConfigsUpdateResponseFieldsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn kind(mut self, value: PostV1DeclarationsConfigsUpdateResponseFieldsItemKind) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsConfigsUpdateResponseFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1DeclarationsConfigsUpdateResponseFieldsItemBuilder::key)
    /// - [`kind`](PostV1DeclarationsConfigsUpdateResponseFieldsItemBuilder::kind)
    pub fn build(self) -> Result<PostV1DeclarationsConfigsUpdateResponseFieldsItem, BuildError> {
        Ok(PostV1DeclarationsConfigsUpdateResponseFieldsItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            multiline: self.multiline,
            options: self.options,
        })
    }
}
