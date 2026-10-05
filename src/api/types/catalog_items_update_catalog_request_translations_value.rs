pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsUpdateCatalogRequestTranslationsValue {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ItemsUpdateCatalogRequestTranslationsValue {
    pub fn builder() -> ItemsUpdateCatalogRequestTranslationsValueBuilder {
        <ItemsUpdateCatalogRequestTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsUpdateCatalogRequestTranslationsValueBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl ItemsUpdateCatalogRequestTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsUpdateCatalogRequestTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ItemsUpdateCatalogRequestTranslationsValueBuilder::name)
    pub fn build(self) -> Result<ItemsUpdateCatalogRequestTranslationsValue, BuildError> {
        Ok(ItemsUpdateCatalogRequestTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
        })
    }
}
