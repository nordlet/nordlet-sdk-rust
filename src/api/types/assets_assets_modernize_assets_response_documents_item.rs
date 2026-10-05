pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetsModernizeAssetsResponseDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl AssetsModernizeAssetsResponseDocumentsItem {
    pub fn builder() -> AssetsModernizeAssetsResponseDocumentsItemBuilder {
        <AssetsModernizeAssetsResponseDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsModernizeAssetsResponseDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl AssetsModernizeAssetsResponseDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetsModernizeAssetsResponseDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AssetsModernizeAssetsResponseDocumentsItemBuilder::name)
    /// - [`r#ref`](AssetsModernizeAssetsResponseDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<AssetsModernizeAssetsResponseDocumentsItem, BuildError> {
        Ok(AssetsModernizeAssetsResponseDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
