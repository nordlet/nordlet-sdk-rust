pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetsGetAssetsResponseDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl AssetsGetAssetsResponseDocumentsItem {
    pub fn builder() -> AssetsGetAssetsResponseDocumentsItemBuilder {
        <AssetsGetAssetsResponseDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsGetAssetsResponseDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl AssetsGetAssetsResponseDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetsGetAssetsResponseDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AssetsGetAssetsResponseDocumentsItemBuilder::name)
    /// - [`r#ref`](AssetsGetAssetsResponseDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<AssetsGetAssetsResponseDocumentsItem, BuildError> {
        Ok(AssetsGetAssetsResponseDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
