pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsCreateRequestDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl PostV1AssetsAssetsCreateRequestDocumentsItem {
    pub fn builder() -> PostV1AssetsAssetsCreateRequestDocumentsItemBuilder {
        <PostV1AssetsAssetsCreateRequestDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsCreateRequestDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl PostV1AssetsAssetsCreateRequestDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsCreateRequestDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1AssetsAssetsCreateRequestDocumentsItemBuilder::name)
    /// - [`r#ref`](PostV1AssetsAssetsCreateRequestDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<PostV1AssetsAssetsCreateRequestDocumentsItem, BuildError> {
        Ok(PostV1AssetsAssetsCreateRequestDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
