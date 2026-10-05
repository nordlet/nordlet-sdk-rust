pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetsInputVatAssetsResponseDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl AssetsInputVatAssetsResponseDocumentsItem {
    pub fn builder() -> AssetsInputVatAssetsResponseDocumentsItemBuilder {
        <AssetsInputVatAssetsResponseDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsInputVatAssetsResponseDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl AssetsInputVatAssetsResponseDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetsInputVatAssetsResponseDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AssetsInputVatAssetsResponseDocumentsItemBuilder::name)
    /// - [`r#ref`](AssetsInputVatAssetsResponseDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<AssetsInputVatAssetsResponseDocumentsItem, BuildError> {
        Ok(AssetsInputVatAssetsResponseDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
