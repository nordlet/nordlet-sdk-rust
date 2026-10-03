pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsInputVatResponseDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl PostV1AssetsAssetsInputVatResponseDocumentsItem {
    pub fn builder() -> PostV1AssetsAssetsInputVatResponseDocumentsItemBuilder {
        <PostV1AssetsAssetsInputVatResponseDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsInputVatResponseDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl PostV1AssetsAssetsInputVatResponseDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsInputVatResponseDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1AssetsAssetsInputVatResponseDocumentsItemBuilder::name)
    /// - [`r#ref`](PostV1AssetsAssetsInputVatResponseDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<PostV1AssetsAssetsInputVatResponseDocumentsItem, BuildError> {
        Ok(PostV1AssetsAssetsInputVatResponseDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
