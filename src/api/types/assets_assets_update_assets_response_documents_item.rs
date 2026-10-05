pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetsUpdateAssetsResponseDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl AssetsUpdateAssetsResponseDocumentsItem {
    pub fn builder() -> AssetsUpdateAssetsResponseDocumentsItemBuilder {
        <AssetsUpdateAssetsResponseDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsUpdateAssetsResponseDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl AssetsUpdateAssetsResponseDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetsUpdateAssetsResponseDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AssetsUpdateAssetsResponseDocumentsItemBuilder::name)
    /// - [`r#ref`](AssetsUpdateAssetsResponseDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<AssetsUpdateAssetsResponseDocumentsItem, BuildError> {
        Ok(AssetsUpdateAssetsResponseDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
