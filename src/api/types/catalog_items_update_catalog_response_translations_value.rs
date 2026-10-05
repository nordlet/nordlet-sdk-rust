pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsUpdateCatalogResponseTranslationsValue {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ItemsUpdateCatalogResponseTranslationsValue {
    pub fn builder() -> ItemsUpdateCatalogResponseTranslationsValueBuilder {
        <ItemsUpdateCatalogResponseTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsUpdateCatalogResponseTranslationsValueBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl ItemsUpdateCatalogResponseTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsUpdateCatalogResponseTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ItemsUpdateCatalogResponseTranslationsValueBuilder::name)
    pub fn build(self) -> Result<ItemsUpdateCatalogResponseTranslationsValue, BuildError> {
        Ok(ItemsUpdateCatalogResponseTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
        })
    }
}
