pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsCreateCatalogRequestTranslationsValue {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ItemsCreateCatalogRequestTranslationsValue {
    pub fn builder() -> ItemsCreateCatalogRequestTranslationsValueBuilder {
        <ItemsCreateCatalogRequestTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsCreateCatalogRequestTranslationsValueBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl ItemsCreateCatalogRequestTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsCreateCatalogRequestTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ItemsCreateCatalogRequestTranslationsValueBuilder::name)
    pub fn build(self) -> Result<ItemsCreateCatalogRequestTranslationsValue, BuildError> {
        Ok(ItemsCreateCatalogRequestTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
        })
    }
}
