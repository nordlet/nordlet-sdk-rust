pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsCreateResponseDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl PostV1AssetsAssetsCreateResponseDocumentsItem {
    pub fn builder() -> PostV1AssetsAssetsCreateResponseDocumentsItemBuilder {
        <PostV1AssetsAssetsCreateResponseDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsCreateResponseDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl PostV1AssetsAssetsCreateResponseDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsCreateResponseDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1AssetsAssetsCreateResponseDocumentsItemBuilder::name)
    /// - [`r#ref`](PostV1AssetsAssetsCreateResponseDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<PostV1AssetsAssetsCreateResponseDocumentsItem, BuildError> {
        Ok(PostV1AssetsAssetsCreateResponseDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
