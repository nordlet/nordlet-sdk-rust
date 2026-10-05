pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsListCatalogResponseRowsItemTranslationsValue {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ItemsListCatalogResponseRowsItemTranslationsValue {
    pub fn builder() -> ItemsListCatalogResponseRowsItemTranslationsValueBuilder {
        <ItemsListCatalogResponseRowsItemTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsListCatalogResponseRowsItemTranslationsValueBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl ItemsListCatalogResponseRowsItemTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsListCatalogResponseRowsItemTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ItemsListCatalogResponseRowsItemTranslationsValueBuilder::name)
    pub fn build(self) -> Result<ItemsListCatalogResponseRowsItemTranslationsValue, BuildError> {
        Ok(ItemsListCatalogResponseRowsItemTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
        })
    }
}
