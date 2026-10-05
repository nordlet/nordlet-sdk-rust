pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UnitsOptionsCatalogResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    pub source: UnitsOptionsCatalogResponseRowsItemSource,
}

impl UnitsOptionsCatalogResponseRowsItem {
    pub fn builder() -> UnitsOptionsCatalogResponseRowsItemBuilder {
        <UnitsOptionsCatalogResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitsOptionsCatalogResponseRowsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    source: Option<UnitsOptionsCatalogResponseRowsItemSource>,
}

impl UnitsOptionsCatalogResponseRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn source(mut self, value: UnitsOptionsCatalogResponseRowsItemSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnitsOptionsCatalogResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](UnitsOptionsCatalogResponseRowsItemBuilder::code)
    /// - [`name`](UnitsOptionsCatalogResponseRowsItemBuilder::name)
    /// - [`source`](UnitsOptionsCatalogResponseRowsItemBuilder::source)
    pub fn build(self) -> Result<UnitsOptionsCatalogResponseRowsItem, BuildError> {
        Ok(UnitsOptionsCatalogResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
