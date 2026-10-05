pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetsUpdateAssetsRequestDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl AssetsUpdateAssetsRequestDocumentsItem {
    pub fn builder() -> AssetsUpdateAssetsRequestDocumentsItemBuilder {
        <AssetsUpdateAssetsRequestDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsUpdateAssetsRequestDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl AssetsUpdateAssetsRequestDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetsUpdateAssetsRequestDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AssetsUpdateAssetsRequestDocumentsItemBuilder::name)
    /// - [`r#ref`](AssetsUpdateAssetsRequestDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<AssetsUpdateAssetsRequestDocumentsItem, BuildError> {
        Ok(AssetsUpdateAssetsRequestDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
