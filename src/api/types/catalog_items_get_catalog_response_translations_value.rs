pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsGetCatalogResponseTranslationsValue {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ItemsGetCatalogResponseTranslationsValue {
    pub fn builder() -> ItemsGetCatalogResponseTranslationsValueBuilder {
        <ItemsGetCatalogResponseTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsGetCatalogResponseTranslationsValueBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl ItemsGetCatalogResponseTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsGetCatalogResponseTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ItemsGetCatalogResponseTranslationsValueBuilder::name)
    pub fn build(self) -> Result<ItemsGetCatalogResponseTranslationsValue, BuildError> {
        Ok(ItemsGetCatalogResponseTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
        })
    }
}
