pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetsDisposeAssetsResponseDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl AssetsDisposeAssetsResponseDocumentsItem {
    pub fn builder() -> AssetsDisposeAssetsResponseDocumentsItemBuilder {
        <AssetsDisposeAssetsResponseDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsDisposeAssetsResponseDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl AssetsDisposeAssetsResponseDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetsDisposeAssetsResponseDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AssetsDisposeAssetsResponseDocumentsItemBuilder::name)
    /// - [`r#ref`](AssetsDisposeAssetsResponseDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<AssetsDisposeAssetsResponseDocumentsItem, BuildError> {
        Ok(AssetsDisposeAssetsResponseDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
